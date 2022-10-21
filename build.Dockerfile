# Arguments for the container
ARG action_name actions

# https://hub.docker.com/_/rust
FROM rust:alpine as builder

WORKDIR /app
COPY . .

# Install openssl dev lib for Hubcaps
RUN apk update && \
    apk add git gcc g++ zlib-dev openssl-dev pkgconfig

# Build and Install using Cargo
RUN cargo build --release


# Production container image
FROM alpine:3.16

COPY --from=builder /app/target/release/$action_name /usr/bin/$action_name

