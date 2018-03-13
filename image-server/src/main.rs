#[macro_use]
extern crate failure;
extern crate glib;
#[macro_use]
extern crate gstreamer as gst;
extern crate gstreamer_app as gst_app;
extern crate ws;

mod camera;
mod error;

use std::cell::Cell;
use std::env;
use std::rc::Rc;
use std::sync::mpsc::channel;
use std::thread;

use ws::{listen, CloseCode, Error as WsError, Handler, Handshake, Message, Sender};

use camera::{Camera, CameraState, Rx};

struct Server {
    camera: Camera,
    count: Rc<Cell<u32>>,
}

impl Server {
    fn new(sender: Sender) -> Server {
        let (send_tx, receive_tx) = channel();
        let server = Server {
            camera: Camera::new(send_tx),
            count: Rc::new(Cell::new(0)),
        };

        // start the receiver thread; this runs forever
        server.start_receiver(sender, receive_tx);

        server
    }

    fn start_receiver(&self, sender: Sender, receiver: Rx) {
        thread::spawn(move || {
            for image in receiver.iter() {
                sender.broadcast(Message::Binary(image)).unwrap_or(());
            }
        });
    }
}

impl Handler for Server {
    fn on_open(&mut self, _: Handshake) -> Result<(), WsError> {
        self.count.set(self.count.get() + 1);

        // if the camera is not running then start it up
        if *self.camera.state() == CameraState::Idle {
            self.camera.start_capture()?;
        }

        Ok(())
    }

    fn on_close(&mut self, _: CloseCode, _: &str) {
        self.count.set(self.count.get() - 1);

        // if the last client has gone away, stop capturing
        if self.count.get() == 0 {
            self.camera.stop_capture().unwrap_or(());
        }
    }
}

fn main() {
    let url = &format!(
        "{}:{}",
        env::var("HOST").unwrap_or("0.0.0.0".to_string()),
        env::var("PORT").unwrap_or("3012".to_string())
    );
    listen(url, |sender| Server::new(sender)).unwrap();
}
