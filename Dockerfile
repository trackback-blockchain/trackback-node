FROM paritytech/ci-linux:production

WORKDIR /var/www/node-template

ADD . /var/www/node-template

RUN rustup toolchain uninstall nightly
RUN rustup toolchain install nightly-2020-08-23
RUN rustup target add wasm32-unknown-unknown --toolchain nightly-2020-08-23

RUN bash -c "cargo build --release"


CMD ["./target/release/node-template", "--dev", "--ws-external"]