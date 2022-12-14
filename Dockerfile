FROM rust:1.63.0 AS chef
WORKDIR /app
RUN cargo install cargo-chef

FROM chef as prepare
COPY . .
# build dependency list
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=prepare /app/recipe.json recipe.json
# build dependencies
RUN cargo chef cook --release --recipe-path recipe.json
# build app
COPY . .
RUN cargo build --release


FROM debian:buster-slim
WORKDIR app
RUN apt update && apt install -y libpq-dev
COPY --from=builder /app/target/release/url_shortener_rust .
ENV RUST_LOG=debug
ENTRYPOINT ["/app/url_shortener_rust"]