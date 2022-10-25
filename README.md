# beam-shader-mvp
A minimum viable dapp built for on Beam

# Usage
1. Install `rustup` on your system. See rust installation instructions [here](https://forge.rust-lang.org/infra/other-installation-methods.html#other-ways-to-install-rustup).
2. Install rust toolchain:
  `$ rustup toolchain install stable`
3. Add wasm32-wasi target
  `$ rustup target add wasm32-wasi`
4. Compile the project
  `$ cargo build --target wasm32-wasi -r`
  Target optional, as defined in `.cargo/config.toml`
5. Compiled wasm files will be in `./target/wasm32-wasi/release` directory

After that you can use `app.wasm` and `contract.wasm` files in the same way you use it in Beam's contracts (see https://github.com/BeamMW/shader-sdk/wiki/Running-Beam-Shaders-using-CLI-Wallet).

`wasm-opt --strip-debug --strip-dwarf --disable-memory64 -Oz -o ./target/wasm32-wasi/release/app.min.wasm  ./target/wasm32-wasi/release/app.wasm`