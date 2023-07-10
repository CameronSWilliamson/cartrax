FROM rust:1-buster as build
WORKDIR /app
COPY . /app
RUN cargo build --release
RUN cargo test

FROM debian:buster
WORKDIR /app
COPY --from=build /app/target/release/cartrax /app/
EXPOSE 8080
CMD [ "./cartrax" ]
