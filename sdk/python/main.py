import logging

from embeddings import EmbeddingService, GrpcEmbeddingClient

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s %(levelname)s [%(name)s] %(message)s",
)

if __name__ == "__main__":
    HOST = "localhost"
    PORT = 50051

    client: EmbeddingService = GrpcEmbeddingClient(HOST, PORT)

    texts = [
        "hello",
        "from",
        "the",
        "other",
        "side",
    ]

    embeddings = client.encode_text(texts)

    for e in embeddings:
        print(e)
