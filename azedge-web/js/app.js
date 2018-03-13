document.addEventListener('DOMContentLoaded', function(event) {
    const start = document.querySelector('#start-playback');
    start.addEventListener('click', onStartPlayback);
});

function onStartPlayback() {
    const wsurl = `ws://${window.location.host.split(':')[0]}:3012`;
    const socket = new WebSocket(wsurl);
    const cameraOutput = document.querySelector('#camera-output');

    socket.addEventListener('message', function(event) {
        const url = URL.createObjectURL(event.data);
        cameraOutput.src = url;
    });
}