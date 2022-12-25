FROM rust:alpine

WORKDIR /app

COPY . .

RUN apk add --no-cache bash

SHELL ["/bin/bash", "-c"]

RUN (apk update && apk add nasm curl bc alpine-sdk) > /dev/null

CMD ["/bin/bash", "bin/test"]