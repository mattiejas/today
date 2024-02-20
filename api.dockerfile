FROM --platform=$BUILDPLATFORM rust:1.76 as builder

ENV SQLX_OFFLINE true

RUN apt update && apt upgrade -y
RUN apt install -y g++-aarch64-linux-gnu libc6-dev-arm64-cross libssl-dev pkg-config libudev-dev

RUN rustup target add aarch64-unknown-linux-gnu
RUN rustup toolchain install stable-aarch64-unknown-linux-gnu

ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++

WORKDIR /usr/src/today-api

COPY ./Cargo.toml ./Cargo.toml
COPY ./api ./api
COPY ./migrations ./migrations
COPY ./lib ./lib
COPY ./.sqlx ./.sqlx

RUN cargo build --bin api --release --target aarch64-unknown-linux-gnu

RUN ls -la target/aarch64-unknown-linux-gnu/release

# ---

FROM debian:bookworm-slim

EXPOSE 80

COPY --from=builder /usr/src/today-api/target/aarch64-unknown-linux-gnu/release/api /usr/local/bin/today-api

CMD ["today-api", "--port", "3001", "--host", "0.0.0.0"]
