FROM docker.io/library/node:24.21.0-bookworm-slim AS web
WORKDIR /src/frontend
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci --no-audit --no-fund
COPY frontend/ ./
RUN npm run build

FROM docker.io/library/rust:1.100-bookworm AS build
WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
RUN cargo build --locked --release -p scopewright-service

FROM docker.io/library/rust:1.100-bookworm
COPY --from=web /usr/local/ /usr/local/
RUN useradd --uid 10001 --create-home scopewright && mkdir -p /app/frontend/dist /data /tmp/upstream-target && chown -R scopewright:scopewright /app /data /tmp/upstream-target
WORKDIR /app
COPY --from=build /src/target/release/scopewright-service /usr/local/bin/scopewright-service
COPY --from=web /src/frontend/dist/ frontend/dist/
COPY scripts/upstream-runtime/ scripts/upstream-runtime/
ENV PORT=8080 SCOPEWRIGHT_DATA_PATH=/data/evidence.jsonl SCOPEWRIGHT_PROBE_ROOT=/upstreams
USER scopewright
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/scopewright-service"]
