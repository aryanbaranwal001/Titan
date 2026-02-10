#![allow(unused)]
use prost::Message;

use proto_build::sf::ethereum::r#type::v2::Block;
use proto_build::sf::firehose::v2::Request as FirehoseRequest;
use proto_build::sf::firehose::v2::stream_client::StreamClient;

use async_stream::try_stream;
use futures_util::Stream;

use serde::{Deserialize, Serialize};
use settings::AppConfig;
use std::str::FromStr;

use tonic::codec::CompressionEncoding::Gzip;
use tonic::service::{Interceptor, interceptor::InterceptedService};
use tonic::transport::{Channel, ClientTlsConfig, Endpoint};
use tonic::{Request, Status, metadata::MetadataValue};

#[derive(Serialize, Debug)]
struct AuthRequest {
    api_key: String,
}

#[derive(Deserialize, Debug)]
struct AuthResponse {
    token: String,
}

pub struct AuthInterceptor {
    token: String,
}

/// Injects an authorization header into every outgoing gRPC request.
///
/// This implementation inserts the bearer 'token' into the metadata of request.
///
/// # Errors
/// Returns `Status::unauthenticated` if the token is ill formatted for gRPC metadata
impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        let token_val = format!("Bearer {}", self.token);

        let metadata = MetadataValue::from_str(&token_val)
            .map_err(|_| Status::unauthenticated("Invalid token format"))?;

        request.metadata_mut().insert("authorization", metadata);

        Ok(request)
    }
}

/// Consumes Firehose API key for bearer token.
///
/// Sends a POST request to the specified `auth_endpoint` with the provided
/// Firehose `api_key`. On success, returns the bearer token used for gRPC stream authorization.
///
/// # Errors
/// * The HTTP request fails
/// * Server returns a non-success status code
/// * Response body cannot be deserialized into an [`AuthResponse`].
pub async fn bearer_token(
    api_key: String,
    auth_endpoint: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .post(auth_endpoint)
        .json(&AuthRequest { api_key })
        .send()
        .await?;

    if !response.status().is_success() {
        let err_text = response.text().await?;
        return Err(format!("Auth failed: {}", err_text).into());
    }
    let auth_data: AuthResponse = response.json().await?;
    Ok(auth_data.token)
}

pub type IngestorClient = StreamClient<InterceptedService<Channel, AuthInterceptor>>;

/// Creates a TLS-enabled gRPC connection to the Firehose service and returns a
/// configured [`IngestorClient`].
///
/// The client is built from a `endpoint_url`, uses webpki-roots CAs for
/// TLS, attaches an authentication interceptor with provided bearer `token`,
/// and enables gzip compression for requests and responses.
///
/// # Errors
/// * The `endpoint_url` is malformed or invalid.
/// * TLS initialization or root certificate loading fails.
/// * The initial TCP/TLS handshake with the remote host times out.
pub async fn build_client(
    token: String,
    endpoint_url: &str,
) -> Result<IngestorClient, Box<dyn std::error::Error>> {
    let endpoint = Endpoint::from_shared(endpoint_url.to_string())?
        .tls_config(ClientTlsConfig::new().with_webpki_roots())?;

    let channel = endpoint.connect().await?;

    let interceptor = AuthInterceptor { token };

    let client = StreamClient::with_interceptor(channel, interceptor)
        .accept_compressed(Gzip)
        .send_compressed(Gzip);

    Ok(client)
}

pub async fn stream_blocks(
    mut client: IngestorClient,
    config: AppConfig,
) -> impl Stream<Item = Result<Block, Box<dyn std::error::Error + Send + Sync>>> {
    let request = FirehoseRequest {
        start_block_num: config.block.start_block,
        stop_block_num: config.block.end_block,
        final_blocks_only: config.block.final_blocks_only,
        cursor: "".to_string(),
        transforms: vec![],
    };

    try_stream! {
        let mut stream = client.blocks(request).await?.into_inner();

        while let Some(response) = stream.message().await? {
            if let Some(any_block) = response.block {
                let decoded_block = Block::decode(&any_block.value[..])
                    .map_err(|e| format!("Decoding error: {}", e))?;

                yield decoded_block;
            }
        }
    }
}

// my notes
// 1. what happens when we move a field out of a struct, what happens with that struct?
// 2. look into how map_err is working, should be early to implement
//
// my todos
// 1. make docs for get_blocks
// 2. rename all get_fns
// 3. rename ingestion to ingestor, something more feasible/intuitive
