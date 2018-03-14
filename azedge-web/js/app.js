document.addEventListener("DOMContentLoaded", function(event) {
  // connect to ML server
  getMLSocket(socket => {
    const start = document.querySelector("#start-playback");
    start.addEventListener("click", onStartPlayback);
  });
});

function onStartPlayback() {
  const wsurl = `ws://${window.location.host.split(":")[0]}:3012`;
  const socket = new WebSocket(wsurl);
  const cameraOutput = document.querySelector("#camera-output");

  socket.addEventListener("message", function(event) {
    evalImage(event.data);
    const url = URL.createObjectURL(event.data);
    cameraOutput.src = url;
  });
}

const MLSocket = {
  socket: null,
  state: "closed"
};

function getMLSocket(callback) {
  // open the socket and wait for "open" event if we are
  // not open
  if (MLSocket.state === "closed") {
    const wsurl = `ws://${window.location.host.split(":")[0]}:8765`;
    MLSocket.socket = new WebSocket(wsurl);
    MLSocket.socket.addEventListener("open", () => {
      MLSocket.state = "open";
      callback(MLSocket.socket);
    });

    MLSocket.socket.addEventListener("close", () => {
      MLSocket.state = "closed";
      MLSocket.socket = null;
    });

    MLSocket.socket.addEventListener("message", onMLResult);
  } else {
    callback(MLSocket.socket);
  }
}

// evaluate images only in 2 second intervals
const evalImage = _.throttle(image => MLSocket.socket.send(image), 2000);

function onMLResult(event) {
  const classList = document.querySelector("#match-line").classList;
  if (event.data === "False") {
    classList.remove("match-line-matched");
    classList.add("match-line-failed");
  } else {
    classList.remove("match-line-failed");
    classList.add("match-line-matched");
  }
}
