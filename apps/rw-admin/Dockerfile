# Build stage for ARM
FROM rust:latest AS build-arm
RUN apt-get update && \
    apt-get install -y musl-tools
COPY . /usr/src/app
WORKDIR /usr/src/app
RUN rustup target add aarch64-unknown-linux-musl
RUN cargo build --release --target aarch64-unknown-linux-musl

# Build stage for AMD
FROM rust:latest AS build-amd
RUN apt-get update && \
    apt-get install -y musl-tools
COPY . /usr/src/app
WORKDIR /usr/src/app
RUN cargo build --release

# Final stage
FROM scratch
WORKDIR /app

# Copy ARM binary from build-arm stage
COPY --from=build-arm /usr/src/app/target/aarch64-unknown-linux-musl/release/rw-admin-arm /app/rw-admin

# Copy AMD binary from build-amd stage
COPY --from=build-amd /usr/src/app/target/release/rw-admin-amd /app/rw-admin

# Set the user for running the application
USER 1000

# Set the entrypoint command
CMD ["./rw-admin"]
