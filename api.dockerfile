FROM rust:1.76 as builder

ENV SQLX_OFFLINE true

WORKDIR /usr/src/today-api

COPY . .

RUN ls -la 

RUN cargo build --bin api --release 

# ---

FROM debian:bookworm-slim 

EXPOSE 3001

COPY --from=builder /usr/src/today-api/target/release/api /usr/local/bin/today-api

CMD ["today-api"]
