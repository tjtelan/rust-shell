FROM alpine:edge as builder
WORKDIR /rust/src/
RUN apk --no-cache add \
  cargo \
  rust
COPY . .
RUN cargo build --release && cargo test

FROM alpine:edge
RUN apk --no-cache add libgcc
COPY --from=builder /rust/src/target/release/rust-shell /usr/local/bin/
CMD ["rust-shell"]
