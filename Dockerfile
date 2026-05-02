FROM rust:1.94.1

# Equivalent to `cd app` - created directory `app` if doesn't exist.
WORKDIR /app
RUN apt update && apt install lld clang -y
# Copy all files from the working environment to the Docker image
COPY . .

ENV SQLX_OFFLINE=true
RUN cargo build --release
ENV APP_ENVIRONMENT="production"
ENTRYPOINT ["./target/release/zero2prod"]
