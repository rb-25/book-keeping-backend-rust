FROM rust:1.79.0-alpine as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/release/bookeeping .

CMD ["bookeeping"]