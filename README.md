# wasm-html5-demo


This is a demo of wasm running in the browser.

The source file calculates a plot of recursion bounding for the function
z = z * z * sin(z) + c in the complex plane and draws the pixels on the
canvas using the Rust Macroquad lib.

The binary size can be minimized by building a release version using the
specified profile in the Cargo.toml and by running the wasm-opt tool.

Prerequisites for build:

- Rust and Cargo
- Python3 and flask
- wasm-opt binary from binaryen

To build and run:

1) cargo build --release --target wasm32-unknown-unknown
2) wasm-opt -Oz -o target/wasm32-unknown-unknown/release/mqtest.wasm target/wasm32-unknown-unknown/release/mqtest.wasm
3) mkdir static
4) cp target/wasm32-unknown-unknown/release/mqtest.wasm static/
5) flask --app demo.py run  --host=0.0.0.0
6) view in any modern browser at http://host:5000 where host is the system running the flask server

After loading the page, you can hold the spacebar down to recalculate with a new color map.

Here is an example rendering:

![z = z * z * sin(z) + c](image/mq.png)
