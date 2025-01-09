# Use a Debian base image with Rust installed
FROM rust:bookworm

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

# Set the working directory
WORKDIR /usr/local/src

# Build MUMPS
RUN git clone https://github.com/coin-or-tools/ThirdParty-Mumps.git
WORKDIR /usr/local/src/ThirdParty-Mumps
RUN ./get.Mumps
RUN mkdir build
RUN ./configure
RUN make -j4
RUN make install -j4

# Build IPOPT
WORKDIR /usr/local/src
RUN git clone https://github.com/coin-or/Ipopt.git
WORKDIR /usr/local/src/Ipopt
RUN mkdir build
RUN ./configure
RUN make -j4
RUN make -j4 test
RUN make -j4 install

# Configure dynamic linker run-time bindings
RUN ldconfig

# Run Rust application
WORKDIR /usr/local/src
COPY . .
RUN cargo build
