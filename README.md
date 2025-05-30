# plist-server

A Rust server used for QuickSign to serve install manifests under HTTPS (as required by iOS).

## How to run

### Binary

#### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Taskfile](https://taskfile.dev/installation/) (Optional)

1. Go into the server's cloned directory, run `task` or `cargo build --release`.
2. Run your binary which is located at `target/release/plist-server`.

### Docker

#### Prerequisites

- [Docker](https://www.docker.com/get-started/)
- [Taskfile](https://taskfile.dev/installation/) (Optional)

1. Go into the server's cloned directory, run `task docker:build` or `docker build -t plist-server .` to build the Docker image.
2. Run the Docker container using `task docker:run` or `docker run -p 8080:8080 plist-server`.

## Acknowledgements

- [plistServer](https://github.com/QuickSign-Team/plistserver)

## License

[MIT](LICENSE) © [TheOrdinaryWow](https://github.com/TheOrdinaryWow)
