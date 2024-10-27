FROM --platform=$BUILDPLATFORM ghcr.io/vladkens/baseimage/rust:latest AS chef
ENV CARGO_INCREMENTAL=0

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json --release --zigbuild \
  --target x86_64-unknown-linux-musl --target aarch64-unknown-linux-musl

COPY . .
RUN cargo zigbuild -r --target x86_64-unknown-linux-musl --target aarch64-unknown-linux-musl && \
  mkdir /app/linux && \
  cp target/aarch64-unknown-linux-musl/release/ogp /app/linux/arm64 && \
  cp target/x86_64-unknown-linux-musl/release/ogp /app/linux/amd64

FROM alpine:latest AS runtime
LABEL org.opencontainers.image.source="https://github.com/vladkens/ogp"

ARG TARGETPLATFORM
ENV HOST=0.0.0.0 PORT=8080

WORKDIR /app
RUN apk add --no-cache ttf-opensans
COPY --from=builder /app/${TARGETPLATFORM} /app/ogp

HEALTHCHECK CMD wget --no-verbose --tries=1 --spider http://127.0.0.1:${PORT}/health || exit 1
EXPOSE ${PORT}

CMD ["/app/ogp"]
