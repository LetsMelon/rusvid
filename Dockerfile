# syntax=docker/dockerfile:1.3-labs

FROM ubuntu:22.10 as base

RUN apt update \
  && apt upgrade -y \
  && apt install -y ffmpeg libavutil-dev libavformat-dev libswscale-dev

FROM base as builder

RUN apt install -y curl gcc pkg-config clang openssl libssl-dev \
  && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly

ENV PATH="$PATH:/root/.cargo/bin"

# TODO move to line 11
RUN apt install -y protobuf-compiler

WORKDIR /rusvid

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release --all
RUN cargo build --release --all

RUN --mount=type=cache,target=/usr/local/cargo/registry <<EOF
  set -e
  echo 'fn hello_world() {}' >> /rusvid/rusvid_server/main.rs
  cargo build --release --all
EOF

FROM base

COPY --from=builder /rusvid/target/release/rusvid_server /rusvid/target/release/rusvid_cli /bin/

CMD ["rusvid_server"]
