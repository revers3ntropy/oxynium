FROM rust:slim-buster

WORKDIR /app

COPY . .

RUN apk add --no-cache bash

SHELL ["/bin/bash", "-c"]

RUN (apt-get update && apt-get install -y nasm curl build-essential bc) > /dev/null

CMD ["/bin/bash", "bin/test"]