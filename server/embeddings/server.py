import logging
from concurrent import futures

import grpc

from . import embeddings_pb2 as pb2
from . import embeddings_pb2_grpc as pb2_grpc


class EmbeddingService(pb2_grpc.EmbeddingServiceServicer):
    """Provides methods that implement functionality of embeddings server."""

    def __init__(self):
        self.logger = logging.getLogger(self.__class__.__name__)

    def EncodeText(
        self,
        request: pb2.TextEmbeddingRequest,
        context: grpc.ServicerContext,
    ) -> pb2.EmbeddingResponse:
        self.logger.info(f"Received request: {request.texts}")

        results = []
        for text in request.texts:
            emb = [float(ord(c)) for c in text]
            results.append(pb2.EmbeddingResult(embeddings=emb))

        return pb2.EmbeddingResponse(results=results)

    def serve(self, executor: futures.Executor):
        server = grpc.server(executor)
        pb2_grpc.add_EmbeddingServiceServicer_to_server(
            self,
            server,
        )

        server.add_insecure_port("[::]:50051")
        server.start()
        self.logger.info("gRPC server started on port 50051")

        server.wait_for_termination()
