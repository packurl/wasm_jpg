![crates.io](https://img.shields.io/crates/v/wasm_jpg.svg)

[WASM](https://developer.mozilla.org/en-US/docs/WebAssembly) libs
for [JPEG](https://en.wikipedia.org/wiki/JPEG) image encoding.

This is a simple wrapper on top of
the [zenjpeg](https://github.com/imazen/zenjpeg) [rust](https://www.rust-lang.org/) [crate](https://crates.io/crates/zenjpeg)
for
encoding a jpeg from rgba8.

<br>

Compilation:

`cargo build --release`

Wasm file optimization:

`wasm-opt --dce --vacuum -Oz target/wasm32-unknown-unknown/release/wasm_jpg.wasm -o jpg.wasm`

<br>

Dependencies:

- [zenjpeg](https://github.com/imazen/heic) ([AGPL3](https://github.com/imazen/zenjpeg/blob/main/LICENSE-AGPL3))
