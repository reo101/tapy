# Build
FROM rust:latest as build

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli
RUN cargo install diesel_cli --no-default-features --features postgres,sqlite

WORKDIR /usr/src/tapy
COPY . .

ARG DATABASE_URL="/usr/src/tapy/databae.db"

RUN cd common && diesel migrations run
RUN cd frontend && trunk build --release
RUN cargo build --release

# Run
FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/src/tapy/target/release/tapy-backend /usr/local/bin/tapy-backend
COPY --from=build /usr/src/tapy/frontend/dist /usr/local/bin/dist

WORKDIR /usr/local/bin

CMD ["tapy-backend"]
