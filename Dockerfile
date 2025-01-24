FROM rust@sha256:62afc139057dc9d3eda02e490677911b55a208ba22d6f7315f3c5c5851e31a36 AS build-stage

LABEL authors="RechellaTek"

RUN mkdir /ez-tickets-api
WORKDIR /ez-tickets-api

COPY Cargo.lock ./Cargo.lock
COPY Cargo.toml ./Cargo.toml

COPY migrations ./migrations
COPY app-cmd ./application-command
COPY application-query   ./application-query
COPY driver ./driver
COPY kernel ./kernel
COPY server ./server

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12

COPY --from=build-stage /ez-tickets-api/target/release/server /

CMD ["/server"]
