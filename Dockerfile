# Build
FROM rust:latest as build

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

WORKDIR /usr/src/tapy
COPY . .

RUN cd frontend && trunk build --release
RUN cargo build --release

# Run
FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/src/tapy/target/release/tapy-backend /usr/local/bin/tapy-backend
COPY --from=build /usr/src/tapy/frontend/dist /usr/local/bin/dist

WORKDIR /usr/local/bin

CMD ["tapy-backend"]
