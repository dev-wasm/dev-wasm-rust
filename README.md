# Devcontainer WASM-Rust
Simple devcontainer for Rust development

# Usage

## Github Codespaces
Just click the button:

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://github.com/codespaces/new?hide_repo_select=true&ref=main&repo=575631038)

## Visual Studio Code
Note this assumes that you have the VS code support for remote containers and `docker` installed 
on your machine.

```sh
git clone https://github.com/dev-wasm/dev-wasm-rust
cd dev-wasm-rust
code ./
```

Visual studio should prompt you to see if you want to relaunch the workspace in a container, you do.

# Building and Running

```sh
rustc main.rs --target wasm32-wasi main.rs
wasmtime main.wasm --dir .
```
