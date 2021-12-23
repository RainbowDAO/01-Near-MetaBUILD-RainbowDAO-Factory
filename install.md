# RainbowDao-ink!


## Installing

Please make sure that you have these prerequisites installed on your computer:

```bash
rustup component add rust-src --toolchain nightly
rustup target add wasm32-unknown-unknown --toolchain stable
```

Then you have to install ink! command line utility which will make setting up Substrate smart contract projects easier:

```bash
cargo install cargo-contract --vers 0.15.0 --force --locked
```

You also need the [binaryen](https://github.com/WebAssembly/binaryen) package installed on your computer which is used to optimize the WebAssembly bytecode of the contract, you can use npm to install it:

```bash
npm install -g binaryen
```

## Testing

First of all you need to clone the repository, run:

```bash
git clone aa
cd SubsCrypt-ink
```

Then, You can enter any folder and enter the following command.

```bash
cargo +nightly test
```

## Building

To build the WASM of your contract and metadata, You can enter any folder and enter the following command.
```bash
cargo +nightly contract build
```


