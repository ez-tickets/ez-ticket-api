FROM rust@sha256:a21d54019c66e3a1e7512651e9a7de99b08f28d49b023ed7220b7fe4d3b9f24e AS build-stage

LABEL authors="RechellaTek"

RUN mkdir /ez-tickets-api
WORKDIR /ez-tickets-api

COPY Cargo.lock ./Cargo.lock
COPY Cargo.toml ./Cargo.toml

COPY migrations ./migrations
COPY application-command ./application
COPY driver ./driver
COPY kernel ./kernel
COPY server ./server

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12

COPY --from=build-stage /ez-tickets-api/target/release/server /

CMD ["/server"]
