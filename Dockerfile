FROM rust

RUN rustup default nightly && rustup update

RUN cargo install cargo-watch

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() { }" > src/main.rs
RUN cargo install

COPY . .
RUN cargo build

CMD cargo run
