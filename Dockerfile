# Build the application
FROM rust:alpine3.17 AS builder
WORKDIR /build

COPY src Cargo.toml Cargo.lock ./


# Run the appilcation
FROM alpine3.17 AS runtime
WORKDIR /var/www/app

COPY --from=builder /build/target/rust-api-example ./

RUN addgroup -S rust && \
	adduser -S rust -G rust && \
	chown -R rust:rust /var/www/app

EXPOSE 8080

CMD ["./rust-api-example"]