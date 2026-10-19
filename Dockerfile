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

# Create a non-privileged system user for security
# Containers should never run as 'root' in production
#
# - Create a group named pgn_grp with a Group ID of 5000
# - Create a user name pgn_usr with an ID of 5000 and add them to the pgn_grp group
# - Do not create a home directory for the user
# - Do not create a group with the same name as the user
RUN groupadd -g 5000 pgn_grp && \
    useradd --no-create-home --no-user-group --uid 5000 --gid pgn_grp pgn_usr

WORKDIR /app

RUN apt-get update && apt-get install -y \
   openssl \
   sqlite3 \
   curl \
   ca-certificates && \ 
   rm -rf /var/lib/apt/lists/*

# Copy ONLY the compiled server binary from the builder stage
COPY --from=builder /app/target/release/techno_penguin /app/

# Copy ONLY the optimized frontend assets (WASM, JS, CSS, Images)
COPY --from=builder --chown=pgn_usr:pgn_grp /app/target/site /app/site

# Optional: If your Leptos app requires a configuration file at runtime, copy it here
COPY --from=builder --chown=pgn_usr:pgn_grp /app/Cargo.toml /app/

# Secure the app directory so it is owned by our non-root user
RUN chown -R pgn_usr:pgn_grp /app

# Switch away from root to the unprivileged user
USER pgn_usr

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT=./site

# Add a healthcheck (optional but recommended)
HEALTHCHECK --interval=60s --timeout=3s \
  CMD curl -f http://localhost:3000/health || exit 1

EXPOSE 3000

# Run the application
CMD ["/app/techno_penguin"]

