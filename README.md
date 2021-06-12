# Yare-Rust

A library for making bots for [yare](https://yare.io) with Rust.

## How it works

You make a Rust library and compile to [wasm](https://webassembly.org/). Then
you use the [wasm2yareio](https://github.com/L0laapk3/yare.io-wasm) script to
generate a JavaScript yare bot. The result can be copied into the game.

## Setup

You will need Rust, and some additional tools.

```bash
rustup toolchain add nightly
rustup target add wasm32-unknown-unknown
rustup component add llvm-tools-preview
rustup update
```

Create a new Rust library.

```bash
cargo new --lib my-yare-bot
```

Put this in your `Cargo.toml`.

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
yare = { git = "https://github.com/ViliamVadocz/yare-rust" }

[profile.release]
opt-level = "s"
lto = true
```

Then copy `wasm2yareio.js` from the linked submodule into your project and
install a dependency. Make sure your Node is up-to-date.

```bash
npm i minify
```

## Example

This is an example bot. You need to define an external function called `tick`
that takes no arguments. This will be called every tick.

```rust
#[no_mangle]
pub extern "C" fn tick(_tick: u32) {
    let pos = unsafe { base::position(0) };
    for index in 0..unsafe { spirit::count() } {
        if unsafe { spirit::is_friendly(index) } {
            unsafe { spirit::goto(index, pos.x, pos.y) };
        }
    }
}
```

## Building

To build your yare bot, you first need to compile to wasm. Use this:

```bash
cargo rustc --target wasm32-unknown-unknown --release -- -C target-feature=+multivalue
```

Then you want to pass it through wasm2yare

```bash
node wasm2yareio target/wasm32-unknown-unknown/release/yarebot.wasm
```
