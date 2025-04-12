import logging
from abc import ABC, abstractmethod

from .sentence_transformer_model import SentenceTransformerModel


class EmbeddingModel(ABC):
    @abstractmethod
    def encode(self, texts: list[str]) -> list[list[float]]:
        pass

    @property
    @abstractmethod
    def model_name(self):
        pass


class ModelRegistry:
    def __init__(self):
        self.models: dict[str, EmbeddingModel] = {}

        self.logger = logging.getLogger(self.__class__.__name__)

        self._register_default()

    def _register_default(self):
        default_model = SentenceTransformerModel()

        self.models["default"] = default_model
        self.models[default_model.model_name] = default_model

        self.logger.info(f"Registered default model: {default_model.model_name}")

    def register(self, name: str, model: EmbeddingModel):
        self.models[name] = model

        self.logger.info(f"Registered default model: {model.model_name}")

    def get(self, name: str = "default") -> EmbeddingModel:
        return self.models[name]
