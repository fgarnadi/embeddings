import logging
from concurrent import futures
from multiprocessing import cpu_count

from embeddings import EmbeddingService

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s %(levelname)s [%(name)s] %(message)s",
)

if __name__ == "__main__":
    executor = futures.ThreadPoolExecutor(max_workers=cpu_count())
    service = EmbeddingService()
    service.serve(executor)
    service.wait_for_termination()
