FROM sickcodes/docker-osx:latest

WORKDIR /app

SHELL ["/bin/bash", "-c"]

# https://brew.sh/
RUN NONINTERACTIVE=1 /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
RUN eval "$(/usr/local/bin/brew shellenv)"
ENV PATH="/usr/local/brew:${PATH}"

RUN brew install -y nasm curl bc gcc

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

COPY . .

CMD ["/bin/bash", "bin/test"]
