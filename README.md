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
    unsafe {
        let me = player::me();
        let (x, y) = unsafe { (base::position_x(0), base::position_y(0)) };
        for index in 0..spirit::count() {
            if spirit::player_id(index) == me && spirit::hp(index) > 0 {
                spirit::goto(index, x, y);
            }
        }
    }
}
```

You should make safe wrappers for these function so that you don't have to have `unsafe` blocks in your bot code.
For example, you could do something like this:

```rust
use std::ffi::CString;
use yare::spirit;
use yare::player;

/// Your own Spirit struct with all the information you want.
struct Spirit {
    index: usize,
    alive: bool,
    friendly: bool,
}

impl Spirit {
    /// Safe wrapper for moving a spirit.
    fn goto(&self, x: f32, y: f32) {
        unsafe { spirit::goto(self.index, x, y) }
    }

    /// Safe wrapper for using shout.
    fn shout(&self, string: &str) {
        let c_string = CString::new(string).unwrap();
        unsafe { spirit::shout(self.index, c_string.as_ptr()) }
    }
}

/// Parse all spirits into your own Spirit structs.
fn get_spirits() -> Vec<Spirit> {
    unsafe {
        let me = player::me();
        let count = spirit::count();
        let mut spirits = Vec::with_capacity(count);
        for index in 0..count {
            spirits.push(Spirit {
                index,
                alive: spirit::hp(index) > 0,
                friendly: spirit::player_id(index) == me,
            });
        }
        spirits
    }
}

// No unsafe block needed here!
#[no_mangle]
pub extern "C" fn tick(_tick: u32) {
    let all_spirits = get_spirits();
    for spirit in all_spirits {
        if spirit.friendly && spirit.alive {
            spirit.goto(123., 456.);
            spirit.shout("Hello, world!");
        }
    }
}
```

## Building

To build your yare bot, you first need to compile to wasm. Use this:

```bash
cargo rustc --target wasm32-unknown-unknown --release
```

Then you want to pass it through wasm2yareio.

```bash
node wasm2yareio target/wasm32-unknown-unknown/release/my-yare-bot.wasm
```
