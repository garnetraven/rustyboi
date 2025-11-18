FROM rust:1.91.1

WORKDIR /usr/src/rustyboi

RUN cargo install cargo-watch

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy the source code
COPY . .

# Expose nothing (Discord uses outbound connections)
CMD ["cargo", "watch", "x", "run"]
