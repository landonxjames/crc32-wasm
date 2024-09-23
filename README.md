### CRC32 WASM

A quick POC of using the Rust [crc32fast](https://crates.io/crates/crc32fast) crate in a WASM component. To run the test code make sure you have `npm install`ed and have both the [Rust toolchain](https://www.rust-lang.org/tools/install) and [cargo component](https://github.com/bytecodealliance/cargo-component) installed.

Then run the following the bulid the WASM binary and the js bindings:

```
npm run build-js-bindings
```

And to run the test code (in `index.ts`) run:

```
npx tsx ./index.ts
```
