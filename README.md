
<p>
  <a href="https://trackback.co.nz/">
    <img src="https://user-images.githubusercontent.com/2051324/127407635-236f8a7a-4ca6-410a-9fc4-add396743cfa.png" alt="TrackBack"></a>
</p>

Blockchain for decentralised identifiers and verifiable credentials.
<br>
<a href="https://github.com/paritytech/substrate/tree/v3.0.0" target="_blank">
    <img src="https://img.shields.io/badge/Substrate-3.0.0-green" alt="Substrate 3.0.0">
</a>
<a href="" target="_blank">
    <img src="https://img.shields.io/badge/build-pass-blueviolet" alt="Codeshare 3.0.0">
</a>
<a href="https://github.com/paritytech/substrate/tree/v3.0.0" target="_blank">
    <img src="https://img.shields.io/badge/terraform-1.0.0-8ca" alt="Substrate 3.0.0">
</a>
## Features
* DID Pallet
* Verifiable Credential Pallet (Late 2021)

## Setting up the chain
* [Install](https://substrate.dev/docs/en/knowledgebase/getting-started/) substrate 
```bash

sudo apt update
# May prompt for location information
sudo apt install -y git clang curl libssl-dev llvm libudev-dev

curl https://getsubstrate.io -sSf | bash -s -- --fast

rustup default stable
rustup update
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```
* Build the project
```bash
cargo build --release
```

## Run

### Single Node Development Chain

Purge any existing dev chain state:

```bash
./target/release/trackback-node purge-chain --dev
```

Start a dev chain:

```bash
./target/release/trackback-node--dev
```

Or, start a dev chain with detailed logging:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/trackback-node -lruntime=debug --dev
```

### Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action, refer to
[our Start a Private Network tutorial](https://substrate.dev/docs/en/tutorials/start-a-private-network/).


# Local deployment

```bash
# Purge any chain data from previous runs
# You will be prompted to type `y`
./target/release/trckback-node purge-chain --base-path /tmp/alice --chain local
```

```bash
# Start Alice's node
./target/release/trackback-node \
  --base-path /tmp/alice \
  --chain local \
  --alice \
  --port 30333 \
  --ws-port 9944 \
  --rpc-port 9933 \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
  --validator
```
Starts Bob's node
```bash
./target/release/trackback-node purge-chain --base-path /tmp/bob --chain local
```

```bash
./target/release/trackback-node \
  --base-path /tmp/bob \
  --chain local \
  --bob \
  --port 30334 \
  --ws-port 9946 \
  --rpc-port 9934 \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
  --validator \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
```

### CLIon configuration
```bash
run --bin trackback-node -- --base-path /tmp/alice --chain local --alice --port 30333 --ws-port 9944 --rpc-port 9933 --node-key 0000000000000000000000000000000000000000000000000000000000000001 --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" --validator
```

### Gets a full list of available APIs for the node
```bash
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "rpc_methods"}' http://localhost:9933/
```
