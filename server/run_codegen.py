# https://github.com/grpc/grpc/blob/master/examples/python/route_guide/run_codegen.py
"""Runs protoc with the gRPC plugin to generate messages and gRPC stubs."""

import glob
from grpc_tools import protoc

proto_files = glob.glob("./proto/*.proto")

protoc.main(
    (
        "",
        "-Iembeddings=./proto",
        "--python_out=.",
        "--grpc_python_out=.",
        "--pyi_out=.",
        *proto_files,
    )
)
