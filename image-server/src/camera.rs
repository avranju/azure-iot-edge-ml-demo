use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::{Receiver, Sender};

use gst;
use gst::prelude::*;
use gst_app;

use error::{ErrorKind, Result};

pub type Tx = Sender<Vec<u8>>;
pub type Rx = Receiver<Vec<u8>>;

#[derive(PartialEq)]
pub enum CameraState {
    Idle,
    Capturing,
}

pub struct Camera {
    state: CameraState,
    sender: Arc<Mutex<Tx>>,
    pipeline: Option<gst::Pipeline>,
}

impl Camera {
    pub fn new(sender: Tx) -> Camera {
        Camera {
            state: CameraState::Idle,
            sender: Arc::new(Mutex::new(sender)),
            pipeline: None,
        }
    }

    pub fn pipeline(&self) -> Option<&gst::Pipeline> {
        self.pipeline.as_ref()
    }

    pub fn state(&self) -> &CameraState {
        &self.state
    }

    pub fn start_capture(&mut self) -> Result<()> {
        if self.state != CameraState::Capturing {
            let pipeline = self.create_pipeline()?;
            pipeline
                .set_state(gst::State::Playing)
                .into_result()
                .map_err(|err| ErrorKind::SystemError(Box::new(err)))?;

            self.pipeline = Some(pipeline);
            self.state = CameraState::Capturing;
        }

        Ok(())
    }

    pub fn stop_capture(&mut self) -> Result<()> {
        if self.state == CameraState::Capturing {
            self.pipeline()
                .map(|pipeline| pipeline.set_state(gst::State::Null).into_result())
                .unwrap()
                .map_err(|err| ErrorKind::SystemError(Box::new(err)))?;

            self.pipeline = None;
            self.state = CameraState::Idle;
        }

        Ok(())
    }

    fn create_pipeline(&self) -> Result<gst::Pipeline> {
        gst::init().map_err(|err| ErrorKind::SystemError(Box::new(err)))?;

        let src =
            gst::ElementFactory::make("v4l2src", None).ok_or(ErrorKind::MissingElement("v4l2src"))?;
        let jpeg =
            gst::ElementFactory::make("jpegenc", None).ok_or(ErrorKind::MissingElement("jpegenc"))?;
        let sink =
            gst::ElementFactory::make("appsink", None).ok_or(ErrorKind::MissingElement("appsink"))?;

        let pipeline = gst::Pipeline::new(None);
        pipeline
            .add_many(&[&src, &jpeg, &sink])
            .map_err(|err| ErrorKind::SystemError(Box::new(err)))?;
        src.link(&jpeg)
            .map_err(|err| ErrorKind::SystemError(Box::new(err)))?;
        jpeg.link(&sink)
            .map_err(|err| ErrorKind::SystemError(Box::new(err)))?;

        let appsink = sink.clone()
            .dynamic_cast::<gst_app::AppSink>()
            .expect("Sink should have been an appsink but wasn't :-/");

        let sender_ref = self.sender.clone();
        let callbacks = gst_app::AppSinkCallbacks::new()
            .new_sample(move |appsink| {
                let sample = match appsink.pull_sample() {
                    None => return gst::FlowReturn::Eos,
                    Some(sample) => sample,
                };

                let buffer = if let Some(buffer) = sample.get_buffer() {
                    buffer
                } else {
                    gst_element_error!(
                        appsink,
                        gst::ResourceError::Failed,
                        ("Failed to get buffer from appsink")
                    );

                    return gst::FlowReturn::Error;
                };

                let map = if let Some(map) = buffer.map_readable() {
                    map
                } else {
                    gst_element_error!(
                        appsink,
                        gst::ResourceError::Failed,
                        ("Failed to map buffer readable")
                    );

                    return gst::FlowReturn::Error;
                };

                // TODO: See if we can avoid this copy here (to_vec()).
                if let Err(_) = sender_ref.lock().unwrap().send(map.as_slice().to_vec()) {
                    gst_element_error!(
                        appsink,
                        gst::ResourceError::Failed,
                        ("Failed to send buffer to channel")
                    );

                    return gst::FlowReturn::Error;
                }

                gst::FlowReturn::Ok
            })
            .build();
        appsink.set_callbacks(callbacks);

        Ok(pipeline)
    }
}
