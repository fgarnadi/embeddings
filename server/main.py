import logging
from concurrent import futures
from multiprocessing import cpu_count

from embeddings import EmbeddingService
from embeddings.models import ModelRegistry

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s %(levelname)s [%(name)s] %(message)s",
)

if __name__ == "__main__":
    model_registry = ModelRegistry()
    service = EmbeddingService(model_registry)

    executor = futures.ThreadPoolExecutor(max_workers=cpu_count())
    service.serve(executor)

    service.wait_for_termination()
