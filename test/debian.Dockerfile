FROM rust:slim-buster

WORKDIR /app

COPY . .

SHELL ["/bin/bash", "-c"]

RUN apt-get update && apt-get install -y nasm curl build-essential bc

CMD ["/bin/bash", "bin/test"]