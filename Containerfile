FROM rust:1
WORKDIR /app
COPY Cargo.toml Cargo.lock .
RUN mkdir src; echo "fn main() {}" > src/main.rs
RUN cargo build
RUN rm ./src/main.rs
COPY . .
ENTRYPOINT ["./entrypoint.sh"]
CMD ["cargo", "test"]
