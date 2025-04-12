from abc import ABC, abstractmethod


class EmbeddingModel(ABC):
    @abstractmethod
    def encode(self, texts: list[str]) -> list[list[float]]:
        pass

    @property
    @abstractmethod
    def model_name(self):
        pass
