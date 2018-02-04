FROM alpine:edge as builder
WORKDIR /rust/src/
RUN apk --no-cache add \
  cargo \
  rust \
  tini
COPY . .
RUN cargo build --release && cargo test

FROM alpine:edge
RUN apk --no-cache add libgcc
COPY --from=builder /rust/src/target/release/rust_shell /usr/local/bin/
COPY --from=builder /sbin/tini /sbin
CMD ["/sbin/tini", "--", "rust_shell"]
