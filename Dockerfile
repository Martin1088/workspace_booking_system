FROM lukemathwalker/cargo-chef:latest-rust-bookworm AS chef
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates curl pkg-config libssl-dev \
  && rm -rf /var/lib/apt/lists/*

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
  && apt-get update \
  && apt-get install -y --no-install-recommends nodejs \
  && rm -rf /var/lib/apt/lists/*

COPY frontend/package*.json ./frontend/
WORKDIR /app/frontend
RUN if [ -f package-lock.json ]; then npm ci; else npm install; fi

WORKDIR /app
COPY . .

WORKDIR /app/frontend
RUN npm run build

WORKDIR /app
RUN cargo build --release --bin workspace_booking_system-cli

FROM gcr.io/distroless/cc-debian12:nonroot AS runtime
WORKDIR /usr/app

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /app/config ./config
COPY --from=builder /app/target/release/workspace_booking_system-cli ./workspace_booking_system-cli
COPY --from=builder /app/frontend/dist/frontend/browser ./frontend/dist/frontend/browser

USER nonroot:nonroot
ENTRYPOINT ["/usr/app/workspace_booking_system-cli"]
CMD ["start", "--environment", "production"]