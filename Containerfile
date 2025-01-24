FROM docker.io/rust:1.84.0-slim-bookworm

# Update and upgrade all base image packages, then install required dependencies
RUN apt-get update && apt-get -y upgrade
RUN apt-get install -y \
    gcc \
    g++ \
    gfortran \
    git \
    patch \
    wget \
    pkg-config \
    liblapack-dev \
    libmetis-dev \
    make \
    cmake

# Build MUMPS
WORKDIR /usr/local/src
RUN git clone https://github.com/coin-or-tools/ThirdParty-Mumps.git
WORKDIR /usr/local/src/ThirdParty-Mumps
RUN ./get.Mumps
RUN mkdir build
RUN ./configure
RUN make -j6
RUN make -j6 install

# Build IPOPT
WORKDIR /usr/local/src
RUN git clone https://github.com/coin-or/Ipopt.git
WORKDIR /usr/local/src/Ipopt
RUN mkdir build
RUN ./configure
RUN make -j6
RUN make -j6 test
RUN make -j6 install

# Configure dynamic linker run-time bindings
RUN ldconfig

# Remove source code for IPOPT and its dependencies
WORKDIR /usr/local/src
RUN rm -rf ThirdParty-Mumps Ipopt

# Create user and home directory for mounting volume from host
RUN useradd -m builder
USER builder
WORKDIR /home/builder
