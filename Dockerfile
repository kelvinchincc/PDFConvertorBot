FROM rust:1.95-slim-bookworm AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    poppler-utils \
    && rm -rf /var/lib/apt/lists/*
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y \
    openssl \
    poppler-utils \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/PDFConvertorBot .
EXPOSE 3000
CMD ["./PDFConvertorBot"]
