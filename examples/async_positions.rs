use aqara::Client;
use aqara::types::{Credentials, Endpoint};

#[tokio::main]
async fn main() -> Result<(), aqara::Error> {
    let client = Client::builder(Credentials::new("APP_ID", "KEY_ID", "APP_KEY"))
        .endpoint(Endpoint::China)
        .access_token("ACCESS_TOKEN")
        .build()?;

    let resp = client.positions().list(Default::default()).await?;
    println!("requestId={} message={}", resp.request_id(), resp.message());
    println!("result={:?}", resp.result());

    Ok(())
}
