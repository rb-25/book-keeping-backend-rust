FROM rust:1.79.0

ENV ROKCKET_ADDRESS = 0.0.0.0
ENV ROCKET_PORT = 8080

WORKDIR /src/bookeeping
COPY . .

RUN rustup default nightly
RUN cargo build

CMD ["cargo", "run"]