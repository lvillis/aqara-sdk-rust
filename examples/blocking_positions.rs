use aqara::BlockingClient;
use aqara::types::{Credentials, Endpoint};

fn main() -> Result<(), aqara::Error> {
    let client = BlockingClient::builder(Credentials::new("APP_ID", "KEY_ID", "APP_KEY"))
        .endpoint(Endpoint::China)
        .access_token("ACCESS_TOKEN")
        .build_blocking()?;

    let resp = client.positions().list(Default::default())?;
    println!("requestId={} message={}", resp.request_id(), resp.message());
    println!("result={:?}", resp.result());

    Ok(())
}
