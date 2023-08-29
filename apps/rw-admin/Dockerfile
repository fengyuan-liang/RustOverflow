# Build stage for ARM
FROM rust:latest AS build-arm
RUN apt update && apt upgrade -y
RUN apt install -y g++-aarch64-linux-gnu libc6-dev-arm64-cross
COPY . /usr/src/app
WORKDIR /usr/src/app
RUN rustup target add aarch64-unknown-linux-gnu
RUN rustup toolchain install stable-aarch64-unknown-linux-gnu
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
    CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc \
    CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++
RUN cargo build --release --target aarch64-unknown-linux-gnu

# Build stage for AMD
FROM rust:latest AS build-amd
RUN apt-get update && apt-get install -y musl-tools && rm -rf /var/lib/apt/lists/*
RUN rustup target add x86_64-unknown-linux-musl && rustup target list
COPY . /usr/src/app
WORKDIR /usr/src/app
RUN cargo build --release --target x86_64-unknown-linux-musl

# Final stage
FROM scratch
WORKDIR /app

# Copy ARM binary from build-arm stage
COPY --from=build-arm /usr/src/app/target/aarch64-unknown-linux-gnu/release/rw-admin /app/rw-admin-arm

# Copy AMD binary from build-amd stage
COPY --from=build-amd /usr/src/app/target/x86_64-unknown-linux-musl/release/rw-admin /app/rw-admin

# Set the user for running the application
USER 1000

# Set the entrypoint command
CMD ["./rw-admin"]