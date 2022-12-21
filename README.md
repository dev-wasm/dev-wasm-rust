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

## Getting Started
`main.rs` is a simple console application that prints a message, writes a file to disk and
then copies that file to a different file

```sh
rustc -g --target wasm32-wasi main.rs
wasmtime main.wasm --dir .
```

## Serving web requests
There is a simple example of web serving via WebAssembly + CGI (WAGI) in
the `webserver` directory. It uses the lighttpd web server and `mod_cgi`.
See the `webserver/lighttpd.conf` file for more details.

```sh
rustc -g --target wasm32-wasi webserver/wagi.rs
lighttpd -D -f webserver/lighttpd.conf
```

lighttpd is configured to serve on port 8080. If you run it in codespaces, it should pop up
a dialog making it super easy to connect to your server. In VS Code it should be
available on http://localhost:8080.