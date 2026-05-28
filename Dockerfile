############################################
# Stage 1: Clone the private repo safely
############################################
FROM alpine/git:latest AS cloner

WORKDIR /app

# Mount the SSH key from your host temporarily during the build
# RUN --mount=type=ssh mkdir -p ~/.ssh && ssh-keyscan github.com >> ~/.ssh/known_hosts
# RUN --mount=type=ssh git clone git@github.com:your-username/your-private-repo.git .

RUN git clone https://github.com/MikeWallaceDev/penguin_site.git  .


############################################
# Stage 2: Build the application
############################################
FROM rustlang/rust:nightly-bookworm AS builder

WORKDIR /app

# Copy the source code strictly from the cloner stage
COPY --from=cloner /app .

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install cargo-leptos
RUN cargo install --locked cargo-leptos

# Install sqlx
RUN cargo install sqlx-cli

# Add the wasm32-unknown-unknown target for hydration
RUN rustup target add wasm32-unknown-unknown

# RUN cargo sqlx prepare
RUN cargo sqlx database setup

# Build the application
# Note: cargo-leptos will also handle Tailwind CSS and SASS compilation
RUN cargo leptos build --release -vv


############################################
# Stage 3: Runtime
############################################
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    openssl \
    sqlite3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/techno_penguin /app/

# Copy the site assets
COPY --from=builder /app/target/site /app/site

# Add a healthcheck (optional but recommended)
HEALTHCHECK --interval=30s --timeout=3s \
  CMD curl -f http://localhost:3000/ || exit 1

# Run the application
CMD ["/app/techno_penguin"]
