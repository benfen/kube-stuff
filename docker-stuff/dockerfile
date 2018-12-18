FROM alpine:3.8

ADD ./target/x86_64-unknown-linux-musl/release/docker-stuff /entrypoint/docker-stuff

ENTRYPOINT ["/entrypoint/docker-stuff"]

EXPOSE 8080