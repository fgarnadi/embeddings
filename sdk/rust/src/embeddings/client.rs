use super::embeddings::{
    EmbeddingResponse, TextEmbeddingRequest, embedding_service_client::EmbeddingServiceClient,
};
use async_trait::async_trait;
use tonic::transport::Channel;

// trait for wrapper
#[async_trait]
pub trait EmbeddingService {
    async fn encode_text(
        &mut self,
        texts: Vec<String>,
    ) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error + Send + Sync>>;
}

pub struct GrpcEmbeddingClient {
    inner: EmbeddingServiceClient<Channel>,
}

impl GrpcEmbeddingClient {
    pub async fn connect(host: &str, port: u16) -> Result<Self, tonic::transport::Error> {
        // TODO : is_secure? https (?)
        let url = format!("http://{host}:{port}");
        let inner = EmbeddingServiceClient::connect(url).await?;
        Ok(Self { inner })
    }
}

#[async_trait]
impl EmbeddingService for GrpcEmbeddingClient {
    async fn encode_text(
        &mut self,
        texts: Vec<String>,
    ) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error + Send + Sync>> {
        let request = tonic::Request::new(TextEmbeddingRequest { texts });
        let response: EmbeddingResponse = self.inner.encode_text(request).await?.into_inner();

        let embeddings = response
            .results
            .into_iter()
            .map(|res| res.embeddings)
            .collect();

        Ok(embeddings)
    }
}
