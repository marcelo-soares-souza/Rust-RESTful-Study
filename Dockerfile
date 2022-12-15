FROM rust:latest

COPY ./ ./

RUN cargo build --release --manifest-path=./tutor-nodb/Cargo.toml

EXPOSE 3000

CMD ["./target/release/tutor-service"]
