# Builder stage
FROM rust:1.94.1 AS builder

# Equivalent to `cd app` - created directory `app` if doesn't exist.
WORKDIR /app
RUN apt update && apt install lld clang -y
# Copy all files from the working environment to the Docker image
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release

# Runtime stage
FROM debian AS runtime
WORKDIR /app

# Install OPENSSL - dynamically linked by some of the dependencies
# Install ca-certificates - verification of TLS certificates when establishing HTTPS connections.
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder environment to the runtime environment
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT="production"
ENTRYPOINT ["./zero2prod"]
