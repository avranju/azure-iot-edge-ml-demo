FROM ubuntu

RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install -y \
      build-essential \
      git \
      lsb-release \
      sudo \
      udev \
      usbutils \
      wget \
    && apt-get clean all

RUN mkdir -p /etc/udev/rules.d/
WORKDIR /code
RUN git clone https://github.com/movidius/ncsdk.git
WORKDIR /code/ncsdk
RUN make install
RUN make examples
