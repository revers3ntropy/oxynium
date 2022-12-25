FROM ubuntu:latest

WORKDIR /app

COPY . .

SHELL ["/bin/bash", "-c"]

RUN (apt-get update && apt-get install -y nasm curl build-essential bc) > /dev/null

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

CMD ["/bin/bash", "bin/test"]