FROM ubuntu:22.10 as base

RUN apt update \
    && apt upgrade -y \
    && apt install -y curl gcc ffmpeg libavutil-dev libavformat-dev libswscale-dev pkg-config clang openssl libssl-dev \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly

ENV PATH="$PATH:/root/.cargo/bin"

RUN cargo install cargo-chef

FROM base as planner

WORKDIR /app

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM base as builder

WORKDIR /rusvid
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json

COPY . .

RUN cargo build -r --all
