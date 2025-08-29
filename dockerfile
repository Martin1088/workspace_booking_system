FROM rust:1.83.0-slim as builder

WORKDIR /usr/src/

COPY . .

RUN apt-get update && apt-get install -y \
    curl \
    ca-certificates \
    libssl3 \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js using the latest available version from NodeSource.
# In production, replace "setup_current.x" with a specific version
# to avoid unexpected breaking changes in future releases.
RUN curl -fsSL https://deb.nodesource.com/setup_current.x | bash - && \
    apt-get install -y nodejs
RUN cd frontend && npm install && npm run build
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/app

COPY docker-entrypoint.sh /usr/app/docker-entrypoint.sh
RUN chmod +x /usr/app/docker-entrypoint.sh

COPY --from=builder /usr/src/frontend/dist/frontend/browser frontend/dist/frontend/browser
COPY --from=builder /usr/src/frontend/dist/frontend/browser/index.html frontend/dist/frontend/browser/index.html
COPY --from=builder /usr/src/config config
COPY --from=builder /usr/src/target/release/workspace_booking_system-cli workspace_booking_system-cli

#ENTRYPOINT ["/usr/app/workspace_booking_system-cli", "start", "--environment", "production"]
ENTRYPOINT ["/usr/app/docker-entrypoint.sh"]
