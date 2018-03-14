FROM avranju/movidius

WORKDIR /app/mlserver/validated_images

ADD validated_images/valid.jpg /app/mlserver/validated_images
ADD face_matcher.py /app/mlserver
ADD server.py /app/mlserver
ADD facenet_celeb_ncs.graph /app/mlserver

# upgrade pip and install dependencies
RUN pip3 install --upgrade pip && \
    pip3 install asyncio && \
    pip3 install websockets

EXPOSE 8765

WORKDIR /app/mlserver
ENTRYPOINT [ "python3", "/app/mlserver/server.py" ]
