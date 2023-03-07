FROM ubuntu:22.10

RUN apt update \
    && apt upgrade -y \
    && apt install -y curl gcc ffmpeg libavutil-dev libavformat-dev libswscale-dev pkg-config clang

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly

COPY . /rusvid
