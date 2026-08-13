// Simple connection test for the Aura API (gRPC) using `aura_api_client`.
// It builds a client, injects the API key via an interceptor, and calls a read‑only RPC (`txn_procs_stat`).
// The response is printed to stdout.

use aura_api_client::{
    client::{AuraClients, UserCtxInterceptor},
    types::AuraUtilsRequest,
};
use log::info;
use std::env;
use tonic::transport::Channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialise logger (env_logger reads RUST_LOG env var)
    env_logger::init();

    // ---- configuration ------------------------------------------------
    // Set your API key via the environment variable `AURA_API_KEY` or replace the placeholder.
    let api_key = env::var("AURA_API_KEY").unwrap_or_else(|_| "YOUR_API_KEY_HERE".to_string());
    let endpoint = "http://trade.aura.rehab:40051";

    // Build a plain‑text tonic channel (no TLS)
    let channel = Channel::from_shared(endpoint.to_string())?.connect().await?;

    // Interceptor that adds the API key to each request
    let interceptor = UserCtxInterceptor::new(api_key);

    // Create the client bundle; we only need the utils RPC.
    let clients = AuraClients::new_with_interceptor(channel, interceptor);

    // Empty request for `txn_procs_stat`
    let request = tonic::Request::new(AuraUtilsRequest::default());

    // Call the RPC
    let response = clients.utils_rpc().txn_procs_stat(request).await?;

    // Show the result
    info!("✅ RPC succeeded – response:");
    println!("{:#?}", response.into_inner());

    Ok(())
}
