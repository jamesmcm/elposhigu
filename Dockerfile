FROM rust:1.47 as builder
WORKDIR app
# Copy over the cached dependencies
COPY . .
# Build our application, leveraging the cached deps!
RUN cargo build --release

FROM debian:buster-slim AS runtime
WORKDIR app
# Install OpenSSL - it is dynamically linked by some of our dependencies
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends ca-certificates
#     && apt-get install -y --no-install-recommends openssl \
#     # Clean up
#     && apt-get autoremove -y \
#     && apt-get clean -y \
#     && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/elposhigu elposhigu
# COPY configuration configuration
# ENV APP_ENVIRONMENT production
ENTRYPOINT ["./elposhigu"]
