FROM ubuntu

# install gstreamer
RUN apt-get update && \
    apt-get install -y gstreamer1.0-plugins-base gstreamer1.0-plugins-good \
      gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly \
      gstreamer1.0-libav

WORKDIR /app
ADD image-server /app

ENV HOST 0.0.0.0
ENV PORT 3012
EXPOSE 3012

ENTRYPOINT [ "/app/image-server" ]
