FROM rust as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM alpine
COPY --from=builder /usr/local/cargo/bin/myapp /app/myapp
CMD ["/app/myapp"]