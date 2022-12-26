FROM sickcodes/docker-osx:latest

WORKDIR /app

COPY . .

SHELL ["/bin/bash", "-c"]

# https://brew.sh/
RUN /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

RUN brew install -y nasm curl build-essential bc

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

CMD ["/bin/bash", "bin/test"]