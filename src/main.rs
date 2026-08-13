use aura_api_client::{
    client::AuraClients,
    UserCtxInterceptor,
    types::TxnProcsStatReq,
};
use std::env;
use tonic::{transport::Channel, Request, Status};

// ─────────────────────────────────────────────────────────
// Struct Ctx yang mengimplementasikan UserCtxInterceptor
// Dibutuhkan sebagai type parameter Ctx di AuraClients<I, Ctx>
// ─────────────────────────────────────────────────────────
#[derive(Clone)]
struct MyCtx;

impl UserCtxInterceptor for MyCtx {
    // Payload = data yang dikirim ke setiap request (API key sebagai String)
    type Payload = String;

    fn intercept<T>(
        api_key: String,
        req: &mut Request<T>,
    ) -> Result<(), Status> {
        req.metadata_mut().insert(
            "x-api-key",
            api_key
                .parse()
                .map_err(|_| Status::unauthenticated("API key tidak valid"))?,
        );
        Ok(())
    }
}

// ─────────────────────────────────────────────────────────
// MAIN
// ─────────────────────────────────────────────────────────
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // API KEY — dibaca dari env var AURA_API_KEY, fallback ke hardcoded
    let api_key = env::var("AURA_API_KEY")
        .unwrap_or_else(|_| "6xuv81QgXfT188g2asvyo6aDc9Zusd1tzFDsUN9nidVX".to_string());

    let endpoint = "http://trade.aura.rehab:40051";

    // Buat channel gRPC plain-text (tanpa TLS)
    let channel = Channel::from_shared(endpoint.to_string())?
        .connect()
        .await?;

    // Interceptor = closure yang menyuntikkan API key ke header setiap request
    // Ini adalah tonic::Interceptor (closure), BUKAN UserCtxInterceptor
    let api_key_for_interceptor = api_key.clone();
    let interceptor = move |mut req: Request<()>| -> Result<Request<()>, Status> {
        req.metadata_mut().insert(
            "x-api-key",
            api_key_for_interceptor.parse().unwrap(),
        );
        Ok(req)
    };

    // Buat AuraClients<I=closure, Ctx=MyCtx>
    let clients: AuraClients<_, MyCtx> = AuraClients::new(channel, interceptor);

    // TxnProcsStatReq adalah struct kosong
    let request = Request::new(TxnProcsStatReq {});

    // Panggil RPC txn_procs_stat
    let response = clients.utils().txn_procs_stat(request).await?;

    println!("✅ Berhasil terhubung ke Aura API!");
    println!("{:#?}", response.into_inner());

    Ok(())
}
