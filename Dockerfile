FROM rust:1.99 AS rust
COPY . /app
WORKDIR /app
RUN cargo build --release

FROM debian:stable-slim
WORKDIR /app

# Install OpenSSL libraries
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=rust /app/target/release/akouthmos /app/akouthmos
COPY --from=rust /app/static /app/static
ENV RUST_BACKTRACE=1
EXPOSE 3000
CMD ["/app/akouthmos"]
