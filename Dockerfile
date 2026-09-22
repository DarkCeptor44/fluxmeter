# stage 1: chef
FROM lukemathwalker/cargo-chef:latest-rust-1.88.0 AS chef
WORKDIR /app

# stage 2: planner
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# stage 3: builder
FROM chef AS builder
RUN apt-get update && apt-get install -y --no-install-recommends curl unzip ca-certificates && \
    curl -fsSL https://bun.sh/install | bash && \
    rm -rf /var/lib/apt/lists/*
ENV PATH="/root/.bun/bin:${PATH}"

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cd frontend && bun install --frozen-lockfile
RUN cargo build --release

# stage 4: final build
FROM gcr.io/distroless/cc-debian12:nonroot
LABEL org.opencontainers.image.title="Fluxmeter" \
      org.opencontainers.image.description="Lightweight speedtest server written in Rust" \
      org.opencontainers.image.licenses="MPL-2.0" \
      org.opencontainers.image.source="https://github.com/DarkCeptor44/fluxmeter"
COPY --from=builder /app/target/release/fluxmeter /usr/local/bin/fluxmeter

USER nonroot:nonroot

ENV FM_HOST=0.0.0.0
ENV FM_PORT=7890

EXPOSE 7890

ENTRYPOINT ["fluxmeter"]
