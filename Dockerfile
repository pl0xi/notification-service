FROM rust:1.82.0-slim-bullseye

WORKDIR /usr/src/app

# Install system dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy the entire project
COPY . .

# Build the application in release mode
RUN cargo build --release

# Run the application
CMD ["./target/release/notification_service"]