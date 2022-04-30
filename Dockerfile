FROM rust:latest
RUN mkdir -p /usr/src/app/
WORKDIR /usr/src/app
RUN rustup component add rustfmt
RUN apt-get update
RUN apt-get install libgtk-3-dev -y
RUN apt-get install libglib2.0-dev -y
RUN apt-get install libcairo2-dev -y
RUN rustup target add x86_64-pc-windows-gnu
RUN rustup toolchain install stable-x86_64-pc-windows-gnu

