FROM archlinux:latest

WORKDIR /app

COPY . .

SHELL ["/bin/bash", "-c"]

RUN pacman -Sy nasm curl base-devel bc --noconfirm

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

CMD ["/bin/bash", "bin/test"]