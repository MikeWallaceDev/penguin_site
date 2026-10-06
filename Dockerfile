# --- SHARED BASE ---
FROM rustlang/rust:nightly-bookworm AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

RUN cargo binstall cargo-generate

# Install cargo-leptos
# RUN cargo install --locked cargo-leptos
# RUN cargo binstall cargo-leptos -y
RUN curl --proto '=https' --tlsv1.3 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

# Install sqlx
RUN cargo binstall sqlx-cli

# --- BUILDER STAGE (Caches Dependencies) ---

# Build actual source
COPY . .

# Create the database
ENV DATABASE_URL=sqlite:/app/data/techno_penguin.db?mode=rwc
RUN mkdir -p data 
RUN sqlx database create
RUN sqlx migrate run --source ./migrations

# Add the wasm32-unknown-unknown target for hydration
RUN rustup target add wasm32-unknown-unknown

# Build the application
# Note: cargo-leptos will also handle Tailwind CSS and SASS compilation
RUN cargo leptos build --release -vv

# --- RUNTIME STAGE ---
FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update && apt-get install -y \
   openssl \
   sqlite3 \
   curl \
   ca-certificates && \ 
   rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/techno_penguin /app/

# If your framework (like Leptos) uses a 'site' package or assets directory, copy it here:
COPY --from=builder /app/target/site /app/site

COPY --from=builder /app/Cargo.toml /app/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT=./site

# Add a healthcheck (optional but recommended)
HEALTHCHECK --interval=30s --timeout=3s \
  CMD curl -f http://localhost:3000/ || exit 1

EXPOSE 3000

# Run the application
CMD ["/app/techno_penguin"]

