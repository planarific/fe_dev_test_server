FROM rust:bookworm as builder

WORKDIR /app
COPY . .
RUN cargo build --release


FROM debian:bookworm-slim AS runner
COPY --from=builder /app/target/release/fe_dev_test_server /app/fe_dev_test_server
EXPOSE 3000
CMD ["/app/fe_dev_test_server"]
