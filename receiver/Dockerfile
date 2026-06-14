# Multi-stage build for the discord-webhook-receiver binary.
#
# IMPORTANT: the build context is the REPO ROOT (not receiver/), because the
# receiver depends on the `discord-webhook-events` lib living at the workspace
# root via `path = ".."`. Build with:
#   docker build -f receiver/Dockerfile -t discord-webhook-receiver .
FROM rust:1-bookworm AS builder
WORKDIR /build
COPY . .
RUN cargo build --release -p discord-webhook-receiver

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /build/target/release/discord-webhook-receiver /usr/local/bin/
EXPOSE 3002
HEALTHCHECK --interval=30s --timeout=3s CMD curl -fsS http://localhost:3002/health || exit 1
ENTRYPOINT ["/usr/local/bin/discord-webhook-receiver"]
