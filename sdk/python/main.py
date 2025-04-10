import logging

from embeddings import EmbeddingService

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s %(levelname)s [%(name)s] %(message)s",
)

if __name__ == "__main__":
    HOST = "localhost"
    PORT = 50051

    client = EmbeddingService(HOST, PORT)

    texts = [
        "hello",
        "from",
        "the",
        "other",
        "side",
    ]

    embeddings = client.EncodeText(texts)

    for e in embeddings:
        print(e)
