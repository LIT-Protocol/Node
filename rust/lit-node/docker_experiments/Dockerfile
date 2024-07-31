# Base image extends rust:nightly which extends debian:buster-slim
FROM rustlang/rust:nightly as build

# Compile Lit Node
RUN rm -rf /tmp/lit_node
RUN mkdir -p /tmp/lit_node
COPY Cargo.* /tmp/lit_node/
COPY src/ /tmp/lit_node/src/
COPY abis/ /tmp/lit_node/abis

WORKDIR /tmp/lit_node

RUN cargo build --release

# Copy the binary into a new container for a smaller docker image
FROM debian:buster-slim

RUN apt-get update
RUN apt-get install -y libssl-dev libsqlite3-dev

COPY --from=build /tmp/lit_node/target/release/lit_node /
USER root

ENV RUST_LOG=info
ENV RUST_BACKTRACE=full
ENV LIT_NODE_PORT=7470
ENV ROCKET_ADDRESS=0.0.0.0
ENV LIT_NODE_DOMAIN_NAME=node3.litgateway.com

EXPOSE 7470

CMD ["/lit_node"]