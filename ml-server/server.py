#! /usr/bin/env python3

import asyncio
import websockets
import cv2
import numpy
import sys
from mvnc import mvncapi as mvnc

from face_matcher import analyze_image, run_inference

GRAPH_FILENAME = "facenet_celeb_ncs.graph"

IMAGES_DIR = './'
VALIDATED_IMAGES_DIR = IMAGES_DIR + 'validated_images/'
validated_image_filename = VALIDATED_IMAGES_DIR + 'valid.jpg'
valid_output = None
graph = None

async def eval_image(websocket, path):
    while True:
        image = await websocket.recv()
        
        # img_bytes = numpy.asarray(bytearray(src), dtype=numpy.uint8)
        img_bytes = numpy.fromstring(image, dtype='uint8')
        img_array = cv2.imdecode(img_bytes, cv2.IMREAD_UNCHANGED)

        match = analyze_image(img_array, valid_output, validated_image_filename, graph)
        await websocket.send(str(match))

def main():
    global valid_output, graph

    # Get a list of ALL the sticks that are plugged in
    # we need at least one
    print("Enumerating devices...")
    devices = mvnc.EnumerateDevices()
    if len(devices) == 0:
        print('No NCS devices found')
        quit()

    # Pick the first stick to run the network
    print("Picking first device...")
    device = mvnc.Device(devices[0])

    # Open the NCS
    print("Opening device...")
    device.OpenDevice()

    # The graph file that was created with the ncsdk compiler
    graph_file_name = GRAPH_FILENAME

    # read in the graph file to memory buffer
    with open(graph_file_name, mode='rb') as f:
        graph_in_memory = f.read()

    # create the NCAPI graph instance from the memory buffer containing the graph file.
    print("Allocating graph instance...")
    graph = device.AllocateGraph(graph_in_memory)

    print("Running inference on valid image...")
    validated_image = cv2.imread(validated_image_filename)
    valid_output = run_inference(validated_image, graph)

    # start websockets server
    print("Starting server...")
    start_server = websockets.serve(eval_image, '0.0.0.0', 8765)
    asyncio.get_event_loop().run_until_complete(start_server)
    asyncio.get_event_loop().run_forever()

    # Clean up the graph and the device
    graph.DeallocateGraph()
    device.CloseDevice()

# main entry point for program. we'll call main() to do what needs to be done.
if __name__ == "__main__":
    sys.exit(main())
