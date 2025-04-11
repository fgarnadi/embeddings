import logging
from abc import ABC, abstractmethod

import grpc

from . import embeddings_pb2 as pb2
from . import embeddings_pb2_grpc as pb2_grpc


class EmbeddingService(ABC):
    @abstractmethod
    def encode_text(self, texts: list[str]) -> list[float]:
        pass


class GrpcEmbeddingClient(EmbeddingService):
    def __init__(self, host: str = "localhost", port: int = 50051):
        self.host = host
        self.port = port

        self.channel: grpc.Channel = grpc.insecure_channel(f"{host}:{port}")
        self.stub: pb2_grpc.EmbeddingServiceServicer = pb2_grpc.EmbeddingServiceStub(
            self.channel
        )

        self.logger = logging.getLogger(self.__class__.__name__)

    def encode_text(self, texts: list[str]) -> list[float]:
        request: pb2.TextEmbeddingRequest = pb2.TextEmbeddingRequest(texts=texts)
        response: pb2.EmbeddingResponse = self.stub.EncodeText(request)

        embeddings = [res.embeddings for res in response.results]
        return embeddings
