# Stage 1: Build Rust Backend
FROM rust:bullseye AS rust-builder

# Set working directory for backend
WORKDIR /usr/src/backend

# Copy backend source code
COPY backend/Cargo.toml backend/Cargo.lock ./
COPY backend/src ./src

# Build the Rust application in release mode with rustls instead of OpenSSL
RUN cargo build --release --no-default-features

# Stage 2: Build Svelte Frontend  
FROM node:current AS svelte-builder

# Enable pnpm
RUN corepack enable pnpm

# Set working directory for frontend
WORKDIR /usr/src/frontend

# Copy frontend package files
COPY frontend/package.json frontend/pnpm-lock.yaml ./

# Install dependencies
RUN pnpm install --frozen-lockfile

# Copy frontend source code
COPY frontend/ ./

# Build the Svelte application
RUN pnpm run build

# Stage 3: Production Runtime
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies (no OpenSSL needed with rustls)
RUN apt-get update && \
    apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy the compiled Rust binary from the first stage
COPY --from=rust-builder /usr/src/backend/target/release/backend /app/backend

# Copy the built Svelte app from the second stage to assets directory
COPY --from=svelte-builder /usr/src/frontend/build /app/assets

# Make the binary executable
RUN chmod +x /app/backend

# Expose port (assuming the Rust app runs on port 3000, adjust if needed)
EXPOSE 3000

# Run the backend application
CMD ["/app/backend"]
