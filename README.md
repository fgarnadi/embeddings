# Embeddings Service

This project is a minimum working example for gRPC service in Python and it's SDK implementations.


## Running the Services

### Prerequisite

Install [uv](https://github.com/astral-sh/uv) for project management.

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