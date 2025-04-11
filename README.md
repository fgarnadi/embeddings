# Embeddings Service

This project is a minimum working example for gRPC service in Python and it's SDK implementations.

## Running the Services

### Prerequisite

Install [uv](https://github.com/astral-sh/uv) for Python project management.  
Install [rustup](https://www.rust-lang.org/tools/install) for rust toolchain and cargo.

### Server

Create virtual environment in `server` directory then install the requirements.

```bash
uv venv
uv sync -U
# activate the venv if needed
# source .venv/bin/activate
```

Run the `main.py` to start the server. The server should be starting.

```bash
uv run main.py
```

### Client

The client is implemented as a simple sdk.

#### Python

Create virtual environment in `sdk/python` directory then install the requirements.

```bash
uv venv
uv sync -U
# activate the venv if needed
# source .venv/bin/activate
```

Run the `main.py` to start the client and do some test request using the sdk.

```bash
uv run main.py
```

#### Rust

Run the `bin/main.rs` from the `sdk/rust` directory using cargo to do some test request using the sdk.  
It will build the binary first if not built yet.

```bash
cargo run --bin main
```
