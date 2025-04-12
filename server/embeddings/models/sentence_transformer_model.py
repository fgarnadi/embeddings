from sentence_transformers import SentenceTransformer

from .interface import EmbeddingModel


class SentenceTransformerModel(EmbeddingModel):
    def __init__(self, model_name: str = "all-MiniLM-L6-v2"):
        self._model_name = model_name
        self.model = SentenceTransformer(model_name)

    def encode(self, texts: list[str]) -> list[list[float]]:
        return self.model.encode(
            texts,
            convert_to_numpy=True,
            show_progress_bar=False,
        ).tolist()

    @property
    def model_name(self):
        return self._model_name
