#![allow(unused)]
use prost::Message;

use proto_build::sf::ethereum::r#type::v2::Block;
use proto_build::sf::firehose::v2::Request as FirehoseRequest;
use proto_build::sf::firehose::v2::stream_client::StreamClient;

use serde::{Deserialize, Serialize};
use std::str::FromStr;

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
pub async fn get_bearer_token(
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
pub async fn get_ingestor_client(
    token: String,
    endpoint_url: &str,
) -> Result<IngestorClient, Box<dyn std::error::Error>> {
    let endpoint = Endpoint::from_shared(endpoint_url.to_string())?
        .tls_config(ClientTlsConfig::new().with_webpki_roots())?;

    let channel = endpoint.connect().await?;

    let interceptor = AuthInterceptor { token };

    let client = StreamClient::with_interceptor(channel, interceptor)
        .accept_compressed(tonic::codec::CompressionEncoding::Gzip)
        .send_compressed(tonic::codec::CompressionEncoding::Gzip);

    Ok(client)
}

pub async fn stream_blocks(mut client: IngestorClient) -> Result<(), Box<dyn std::error::Error>> {
    let request = FirehoseRequest {
        start_block_num: 20_375_440,
        stop_block_num: 20_375_442,
        final_blocks_only: true,
        // cursors are for resuming a stream; leave empty for a new start
        cursor: "".to_string(),
        // We will add filters (transforms) in a later step
        transforms: vec![],
    };

    // This initiates the gRPC streaming call
    let mut stream = client.blocks(request).await?.into_inner();

    let mut block_no: u32 = 0;
    while let Some(response) = stream.message().await? {
        // 1. Firehose sends a 'block' envelope
        if let Some(any_block) = response.block {
            match Block::decode(&any_block.value[..]) {
                Ok(decoded_block) => {
                    println!("Block: {:#?}", decoded_block);
                    // 3. Manual stop check (optional: removing for now)
                    // Firehose servers sometimes send a few extra blocks for reorg safety,
                    // so we break manually when we hit our target.
                }
                Err(e) => eprintln!("Decoding error: {}", e),
            }
            block_no += 1;
        }
    }
    println!("blocks printed: {}", block_no);
    Ok(())
}
