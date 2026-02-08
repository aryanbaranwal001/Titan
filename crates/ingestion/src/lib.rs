#![allow(unused)]
use proto_build::sf::ethereum::r#type::v2::Block;
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

// .post("https://auth.streamingfast.io/v1/auth/issue")
pub async fn get_firehose_token(
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

// let endpoint_url = "https://mainnet.eth.streamingfast.io:443";
type IngestorClient = StreamClient<InterceptedService<Channel, AuthInterceptor>>;
pub async fn connect_to_firehose(
    token: String,
    endpoint_url: &'static str,
) -> Result<IngestorClient, Box<dyn std::error::Error>> {
    let endpoint = Endpoint::from_static(endpoint_url)
        .tls_config(ClientTlsConfig::new().with_webpki_roots())?;

    let channel = endpoint.connect().await?;

    let interceptor = AuthInterceptor { token };

    let client = StreamClient::with_interceptor(channel, interceptor)
        .accept_compressed(tonic::codec::CompressionEncoding::Gzip)
        .send_compressed(tonic::codec::CompressionEncoding::Gzip);

    Ok(client)
}

// my notes: this has nothing to do with the codebase
// Checkout the following for once
// see how box dyn std::error::Error works
// how does .into works
// see how from_str trait works
