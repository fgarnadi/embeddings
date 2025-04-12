from abc import ABC, abstractmethod


class EmbeddingService(ABC):
    @abstractmethod
    def encode_text(self, texts: list[str]) -> list[float]:
        pass
