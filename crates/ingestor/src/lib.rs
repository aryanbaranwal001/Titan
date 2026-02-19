#![allow(unused)]
use prost::Message;
use std::io::ErrorKind;

use tokio::fs::{self, OpenOptions};
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

use proto_build::sf::ethereum::r#type::v2::Block;
use proto_build::sf::firehose::v2::Request as FirehoseRequest;
use proto_build::sf::firehose::v2::stream_client::StreamClient;

use async_stream::try_stream;
use futures_util::Stream;

use serde::{Deserialize, Serialize};
use settings::{AppConfig, BoxBlockErr};
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
        .send_compressed(Gzip)
        .max_decoding_message_size(256 * 1024 * 1024);

    Ok(client)
}

/// Streams and decodes blocks from the Firehose gRPC endpoint per [`AppConfig`]
///
/// It establishes an asynchronous connection, processes raw block messages from the server,
/// and yields a stream of decoded [`Block`] results. The stream automatically handles
/// Protobuf deserialization and propagates network or decoding errors.
///
/// # Errors
/// * The gRPC block request fails to initialize.
/// * The stream is interrupted by network issues.
/// * The raw block data fails to decode into the [`Block`] type.
pub async fn stream_blocks(
    mut client: IngestorClient,
    config: &AppConfig,
) -> impl Stream<Item = Result<Block, Box<dyn std::error::Error + Send + Sync>>> {
    let request = FirehoseRequest {
        start_block_num: config.start_block,
        stop_block_num: config.end_block,
        final_blocks_only: config.final_blocks_only,
        cursor: "".to_string(),
        transforms: vec![],
    };

    try_stream! {
        let mut stream = client.blocks(request).await?.into_inner();

        while let Some(response) = stream.message().await? {
            if let Some(block) = response.block {
                let decoded_block = Block::decode(&block.value[..])
                    .map_err(|e| format!("Decoding error: {}", e))?;

                yield decoded_block;
            }
        }
    }
}

// ingest blocks from firehose api key and store it in blocks.bin file
// to use that data as mock
pub async fn store_blocks(
    mut client: IngestorClient,
    config: &AppConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let request = FirehoseRequest {
        start_block_num: config.start_block,
        stop_block_num: config.end_block,
        final_blocks_only: config.final_blocks_only,
        cursor: "".to_string(),
        transforms: vec![],
    };

    let mut stream = client.blocks(request).await?.into_inner();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("blocks.bin")
        .await?;

    while let Some(response) = stream.message().await? {
        if let Some(block) = response.block {
            let len = block.value.len() as u32;
            file.write_u32(len).await?;
            file.write_all(&block.value).await?;

            let decoded_block =
                Block::decode(&block.value[..]).map_err(|e| -> Box<dyn std::error::Error> {
                    format!("Decoding error: {}", e).into()
                })?;

            println!("saved to disk, block number: {}", decoded_block.number);
        }
    }

    Ok(())
}
// my notes
// 1. what happens when we move a field out of a struct, what happens with that struct?
//
// my todos
