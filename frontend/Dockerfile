FROM rust:1.61.0 AS develop
WORKDIR /want-this-frontend
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
EXPOSE 8080