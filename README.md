# Site for https://osvald.org
Written in Rust using the [Yew](https://yew.rs/) framework which compiles to WebAssembly. This is of course totally overkill for a simple static site like this which could have just been done as 2 HTML files instead...

## How to run locally
1. Install Rust: https://www.rust-lang.org/tools/install
1. Install WebAssembly target: `rustup target add wasm32-unknown-unknown`
1. Install Trunk: `cargo install trunk`
1. Run locally on port 8080: `trunk serve` (see results on http://localhost:8080)

## How to deploy
1. Build release files in `dist/`: `trunk build --release`
2. Host the `dist` folder in any way you want.