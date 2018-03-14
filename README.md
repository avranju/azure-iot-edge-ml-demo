# Azure IoT Edge + Movidius Neural Compute Stick ML Demo App

This app has 3 IoT Edge modules:

  - **Image Server** - this module uses the Linux [GStreamer](https://gstreamer.freedesktop.org/) API to interface with a [Video4Linux](https://en.wikipedia.org/wiki/Video4Linux) compatible camera to sample image frames and make them available on a WebSocket endpoint using JPEG encoding. The WebSocket server listens on port `3012` by default. This has been written using the Rust programming language.

  - **Machine Learning Server** - this module uses the Movidius Neural Compute Stick Python (NCS) [SDK](https://movidius.github.io/ncsdk/) to interface with the NCS stick to run image pattern analysis jobs. This is also a WebSocket server written in Python and listens on port `8765` by default. This module has been adapted from the [face matcher sample](https://github.com/movidius/ncappzoo/tree/master/apps/video_face_matcher) provided as part of the NCS SDK. In order to run this module you'll need to run through the setup instructions as described [here](https://github.com/movidius/ncappzoo/tree/master/apps/video_face_matcher). And you will also, of course, need the NCS USB stick itself.

  - **Web Frontend** - this module is a static website that serves as a web interface for testing the system out. It uses _nginx_ to serve the site and uses the JavaScript websocket API to interface with the image server and the ML server to sample image frames and evaluate them.

