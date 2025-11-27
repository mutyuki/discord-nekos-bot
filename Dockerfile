FROM rust:1.91 as builder

WORKDIR /app

COPY Cargo.toml ./

RUN mkdir src && \
  echo "fn main() {}" > src/main.rs && \
  cargo build --release && \
  rm -rf src

COPY src ./src

RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
  ca-certificates \
  tzdata \
  && rm -rf /var/lib/apt/lists/*

ENV TZ=Asia/Tokyo

COPY --from=builder /app/target/release/discord-nekos-bot /usr/local/bin/discord-nekos-bot

CMD ["discord-nekos-bot"]
