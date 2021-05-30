# WEB ASSEMBLY WITH RUST

## Hey guys
This is a very simple example of using web assembly. I made this based on this [content](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm)


## Requirements
To run this application, you will need Rust and Cargo, to build the Rust code into javascript code, with wasm-pack. Also, you will need to use python or any other language that allows you to create http servers.

## Getting started
To run this app, you will need to run this two commands:
- `wasm-pack build --target web` to build the code into .wasm and .js files;
- PYTHON: `python -m http.server`, to see the result of the command above.
