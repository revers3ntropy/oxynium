FROM sickcodes/docker-osx:latest

WORKDIR /app

SHELL ["/bin/bash", "-c"]

# https://brew.sh/
RUN /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
RUN echo '# Set PATH, MANPATH, etc., for Homebrew.' >> /home/arch/.profile
RUN echo 'eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"' >> /home/arch/.profile
RUN eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"

RUN brew install -y nasm curl bc gcc

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

COPY . .

CMD ["/bin/bash", "bin/test"]