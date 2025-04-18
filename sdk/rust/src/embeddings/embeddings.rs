// This file is @generated by prost-build.
/// Request message for encoding text data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextEmbeddingRequest {
    /// A list of text strings to encode.
    #[prost(string, repeated, tag = "1")]
    pub texts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A single embedding result, representing a vector of float values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmbeddingResult {
    #[prost(float, repeated, tag = "1")]
    pub embeddings: ::prost::alloc::vec::Vec<f32>,
}
/// A response containing one embedding result for each input
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmbeddingResponse {
    /// A list of embedding results, in the same order as input.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<EmbeddingResult>,
}
/// Generated client implementations.
pub mod embedding_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The EmbeddingService provides encoding of input data into numerical
    /// vector representations
    #[derive(Debug, Clone)]
    pub struct EmbeddingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EmbeddingServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> EmbeddingServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> EmbeddingServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::Body>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::Body>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            EmbeddingServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Encodes one or more input texts into vector embeddings.
        pub async fn encode_text(
            &mut self,
            request: impl tonic::IntoRequest<super::TextEmbeddingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EmbeddingResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/embeddings.EmbeddingService/EncodeText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("embeddings.EmbeddingService", "EncodeText"));
            self.inner.unary(req, path, codec).await
        }
    }
}
