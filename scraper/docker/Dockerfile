FROM rust:1.70 AS builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

RUN ls -l target/release

FROM debian:bullseye-slim

COPY --from=builder /usr/src/app/target/release/ziptern-scraper /usr/local/bin/ziptern-scraper
COPY --from=builder /usr/src/app/src/.env /usr/local/bin/.env
COPY --from=builder /usr/src/app/src/constants /usr/local/bin/constants
COPY --from=builder /usr/src/app/src/output /usr/local/bin/output

WORKDIR /usr/local/bin

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

CMD ["ziptern-scraper"]