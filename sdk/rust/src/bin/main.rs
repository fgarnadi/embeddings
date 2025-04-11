use embeddings_sdk::{EmbeddingService, GrpcEmbeddingClient};

use tokio;

#[tokio::main]
async fn main() {
    const HOST: &str = "localhost";
    const PORT: u16 = 50051;

    let mut client = GrpcEmbeddingClient::connect(HOST, PORT).await.unwrap();

    let texts = vec!["hello", "from", "the", "other", "side"]
        .into_iter()
        .map(String::from)
        .collect();

    let embeddings = client.encode_text(texts).await.unwrap();
    for embedding in embeddings {
        println!("{:?}", embedding);
    }
}
