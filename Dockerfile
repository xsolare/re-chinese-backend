FROM rust:latest

RUN apt-get update \
    && echo "installing debugging tools" \
    && apt-get install -y \
        curl \
        procps \
        net-tools \
    && mkdir -p -m 777 /server

ADD ./Cargo.toml /server/Cargo.toml
ADD ./Cargo.lock /server/Cargo.lock
ADD ./src /server/src

# add custom user here in future version

RUN echo "starting build" \
    && cd /server \
    && cargo build --release

EXPOSE 3000
ENTRYPOINT ["/server/target/release/server"]
