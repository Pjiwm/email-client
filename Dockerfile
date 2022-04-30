FROM rust:latest
RUN mkdir -p /usr/src/app/
WORKDIR /usr/src/app
RUN apt-get update
RUN apt-get install netcat -y
