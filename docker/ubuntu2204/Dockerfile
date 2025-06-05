FROM ubuntu:22.04

ENV YARA_TAR_VERSION=4.3.2
ENV DEBIAN_FRONTEND=noninteractive
ENV PATH=/root/yara-${YARA_TAR_VERSION}:/root/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

RUN apt update
RUN apt upgrade -y
RUN apt install -y build-essential autoconf libtool curl git libssl-dev libudev-dev pkg-config ltrace strace cmake protobuf-compiler clang
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN cargo install cargo-audit

WORKDIR /root
RUN curl -sSLO https://github.com/VirusTotal/yara/archive/refs/tags/v${YARA_TAR_VERSION}.tar.gz
RUN tar -zxf v${YARA_TAR_VERSION}.tar.gz
RUN rm v${YARA_TAR_VERSION}.tar.gz

WORKDIR /root/yara-${YARA_TAR_VERSION}
RUN ./bootstrap.sh
RUN ./configure
RUN make
LABEL frozen=yes
