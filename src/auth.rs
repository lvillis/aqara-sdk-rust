use rand::Rng;
use rand::distr::Alphanumeric;

use crate::error::Error;
use crate::types::{Credentials, SecretString};
use crate::util::time::unix_timestamp_millis;

pub(crate) struct SignatureParts {
    pub(crate) nonce: String,
    pub(crate) time_millis: String,
    pub(crate) sign: String,
}

pub(crate) fn sign_headers(
    credentials: &Credentials,
    access_token: Option<&SecretString>,
    include_access_token: bool,
) -> Result<SignatureParts, Error> {
    let nonce = generate_nonce();
    let time_millis = unix_timestamp_millis()?;

    if include_access_token && access_token.is_none() {
        return Err(Error::InvalidConfig {
            message: "access token is required for this operation".to_string(),
        });
    }

    let sign = generate_signature(
        credentials,
        access_token,
        &nonce,
        &time_millis,
        include_access_token,
    );

    Ok(SignatureParts {
        nonce,
        time_millis,
        sign,
    })
}

fn generate_nonce() -> String {
    let rng = rand::rng();
    rng.sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}

fn generate_signature(
    credentials: &Credentials,
    access_token: Option<&SecretString>,
    nonce: &str,
    time: &str,
    include_access_token: bool,
) -> String {
    let mut sign_str = String::new();

    if include_access_token
        && let Some(token) = access_token
        && !token.expose().is_empty()
    {
        sign_str.push_str("Accesstoken=");
        sign_str.push_str(token.expose());
        sign_str.push('&');
    }

    sign_str.push_str("Appid=");
    sign_str.push_str(credentials.app_id());
    sign_str.push_str("&Keyid=");
    sign_str.push_str(credentials.key_id());
    sign_str.push_str("&Nonce=");
    sign_str.push_str(nonce);
    sign_str.push_str("&Time=");
    sign_str.push_str(time);
    sign_str.push_str(credentials.app_key().expose());

    let sign_str = sign_str.to_lowercase();
    let digest = md5::compute(sign_str.as_bytes());
    format!("{digest:x}")
}
