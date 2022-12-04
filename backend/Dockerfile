FROM ghcr.io/steamdeckhomebrew/holo-toolchain-rust:latest

RUN pacman -S --noconfirm git python3
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

ENTRYPOINT [ "/caylon/backend/entrypoint.sh" ]
