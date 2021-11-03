FROM paritytech/ci-linux:105b919b-20210617 as builder

WORKDIR /build

ARG FEATURES=default

COPY ./.git/ /build/.git/
COPY ./node /build/node
COPY ./pallets /build/pallets
COPY ./runtime /build/runtime
COPY ./Cargo.lock /build/Cargo.lock
COPY ./Cargo.toml /build/Cargo.toml

RUN cargo build --release --features $FEATURES


FROM debian:buster-slim
ARG RPC_PORT=9933
ARG WEB_SOCKET_PORT=9944
ARG P2P_PORT=30333
ARG NODE_TYPE=trackback-node

COPY ./LICENSE /build/LICENSE
COPY --from=builder /build/target/release/$NODE_TYPE /usr/local/bin/node-executable

RUN useradd -m -u 1000 -U -s /bin/sh -d /node node && \
	mkdir -p /node/.local/share/node && \
	chown -R node:node /node/.local && \
	ln -s /node/.local/share/node /data && \
	rm -rf /usr/bin /usr/sbin

USER node
EXPOSE ${RPC_PORT} ${P2P_PORT} ${WEB_SOCKET_PORT}
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/node-executable"]
CMD ["--help"]
