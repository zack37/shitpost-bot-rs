FROM rust:1.52 as builder

COPY . /usr/src/app

WORKDIR /usr/src/app

RUN cargo build --release

FROM rust:1.52 as runner

COPY --from=builder /usr/src/app/docker-entrypoint.sh /usr/local/bin
COPY --from=builder /usr/src/app/target/release/shitpost-bot-rs /usr/local/bin

ENTRYPOINT [ "./usr/local/bin/docker-entrypoint.sh" ]
