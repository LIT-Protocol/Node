


FROM --platform=linux/x86_64 ubuntu:22.04 AS ENV

VOLUME [ "/data" ]

WORKDIR /data

COPY --from=docker.io/litptcl/anvil-lit /usr/local/bin/anvil /usr/local/bin
COPY ./shiva/ci/config /app/config
COPY ./shiva/ci/tests /app/tests
COPY ./shiva/ci/rpc-config.yaml /app/rpc-config.yaml
COPY ./start_shiva.sh /app/start_shiva.sh

EXPOSE 8000 8000 
EXPOSE 8545 8545

EXPOSE 7470 7470
EXPOSE 7471 7471
EXPOSE 7472 7472
EXPOSE 7473 7473
EXPOSE 7475 7475
EXPOSE 7475 7475

RUN apt update -y
RUN apt upgrade -y
RUN apt-get install -y curl git libssl-dev libsqlite3-dev libudev-dev jq openssl unzip
RUN apt install protobuf-compiler -y

RUN curl -fsSL https://deb.nodesource.com/setup_18.x -o nodesource_setup.sh
RUN chmod -x nodesource_setup.sh
RUN sh nodesource_setup.sh
RUN apt-get install -y nodejs
ENTRYPOINT ["sh", "/app/start_shiva.sh"]