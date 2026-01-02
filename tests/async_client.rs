#![cfg(feature = "async")]

use std::time::Duration;

use aqara::types::auth::{
    CreateAccountParams, GetAuthCodeParams, GetTokenParams, RefreshTokenParams,
};
use aqara::types::devices::QuerySubDevicesParams;
use aqara::types::positions::ListPositionsParams;
use aqara::types::push::{TraitSubscribeParams, TraitSubscription};
use aqara::types::resources::CommandDeviceResourceParams;
use aqara::types::{Credentials, RetryConfig};
use aqara::{Client, Error};
use serde_json::json;
use wiremock::matchers::{body_json, header, header_exists, method, path};
use wiremock::{Match, Mock, MockServer, Request, ResponseTemplate};

#[derive(Clone)]
struct HeaderAbsent(&'static str);

impl Match for HeaderAbsent {
    fn matches(&self, request: &Request) -> bool {
        !request.headers.contains_key(self.0)
    }
}

#[tokio::test]
async fn retries_idempotent_on_429_retry_after() -> Result<(), Box<dyn std::error::Error>> {
    let server = MockServer::start().await;

    let body = json!({
        "intent": "query.position.info",
        "data": {
            "parentPositionId": "",
            "pageNum": 1,
            "pageSize": 30,
        }
    });

    let _rate_limited_mock = Mock::given(method("POST"))
        .and(path("/v3.0/open/api"))
        .and(body_json(body.clone()))
        .respond_with(
            ResponseTemplate::new(429)
                .append_header("Retry-After", "0")
                .set_body_string("rate limited"),
        )
        .with_priority(1)
        .up_to_n_times(1)
        .expect(1)
        .named("first request gets 429")
        .mount_as_scoped(&server)
        .await;

    let _success_mock = Mock::given(method("POST"))
        .and(path("/v3.0/open/api"))
        .and(header("appid", "APP_ID"))
        .and(header("keyid", "KEY_ID"))
        .and(header("accesstoken", "ACCESS_TOKEN"))
        .and(header("content-type", "application/json"))
        .and(header("lang", "en"))
        .and(header_exists("nonce"))
        .and(header_exists("time"))
        .and(header_exists("sign"))
        .and(body_json(body))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "requestId": "req-1",
            "message": "Success"
        })))
        .with_priority(2)
        .expect(1)
        .named("second request succeeds")
        .mount_as_scoped(&server)
        .await;

    let base_url = format!("{}/v3.0/open/api", server.uri()).parse()?;
    let client = Client::builder(Credentials::new("APP_ID", "KEY_ID", "APP_KEY"))
        .base_url(base_url)
        .access_token("ACCESS_TOKEN")
        .retry(RetryConfig::new(
            1,
            Duration::from_millis(0),
            Duration::from_millis(0),
        ))
        .build()?;

    let resp = client
        .positions()
        .list(ListPositionsParams::default())
        .await?;

    assert!(resp.status.is_success());
    Ok(())
}

#[tokio::test]
async fn does_not_retry_non_idempotent_on_500() -> Result<(), Box<dyn std::error::Error>> {
    let server = MockServer::start().await;

    let body = json!({
        "intent": "command.device.resource",
        "data": {
            "positionId": "pos",
            "queryText": "turn on",
        }
    });

    let _server_error_mock = Mock::given(method("POST"))
        .and(path("/v3.0/open/api"))
        .and(body_json(body))
        .respond_with(ResponseTemplate::new(500).set_body_string("boom"))
        .expect(1)
        .named("non-idempotent returns 500 once")
        .mount_as_scoped(&server)
        .await;

    let base_url = format!("{}/v3.0/open/api", server.uri()).parse()?;
    let client = Client::builder(Credentials::new("APP_ID", "KEY_ID", "APP_KEY"))
        .base_url(base_url)
        .access_token("ACCESS_TOKEN")
        .retry(RetryConfig::new(
            5,
            Duration::from_millis(0),
            Duration::from_millis(0),
        ))
        .build()?;

    let err = match client
        .resources()
        .command_device_resource(CommandDeviceResourceParams::new("pos", "turn on"))
        .await
    {
        Ok(_) => return Err("expected an error".into()),
        Err(e) => e,
    };

    assert!(matches!(err, Error::Http { status, .. } if status.as_u16() == 500));
    Ok(())
}

#[tokio::test]
async fn access_token_is_required_when_endpoint_demands_it()
-> Result<(), Box<dyn std::error::Error>> {
    let server = MockServer::start().await;
    let base_url = format!("{}/v3.0/open/api", server.uri()).parse()?;

    let client = Client::builder(Credentials::new("APP_ID", "KEY_ID", "APP_KEY"))
        .base_url(base_url)
        .build()?;

    let err = match client
        .devices()
        .sub_info(QuerySubDevicesParams::new("gw"))
        .await
    {
        Ok(_) => return Err("expected an error".into()),
        Err(e) => e,
    };

    assert!(matches!(err, Error::InvalidConfig { .. }));
    Ok(())
}

#[tokio::test]
async fn decode_error_includes_http_status() -> Result<(), Box<dyn std::error::Error>> {
    let server = MockServer::start().await;

    let body = json!({
        "intent": "query.position.info",
        "data": {
            "parentPositionId": "",
            "pageNum": 1,
            "pageSize": 30,
        }
    });

    let _mock = Mock::given(method("POST"))
        .and(path("/v3.0/open/api"))
        .and(body_json(body))
        .respond_with(ResponseTemplate::new(200).set_body_string("not json"))
        .expect(1)
        .named("invalid json body")
        .mount_as_scoped(&server)
        .await;

    let base_url = format!("{}/v3.0/open/api", server.uri()).parse()?;
    let client = Client::builder(Credentials::new("APP_ID", "KEY_ID", "APP_KEY"))
        .base_url(base_url)
        .access_token("ACCESS_TOKEN")
        .build()?;

    let err = match client
        .positions()
        .list(ListPositionsParams::default())
        .await
    {
        Ok(_) => return Err("expected an error".into()),
        Err(e) => e,
    };

    assert!(matches!(err, Error::Decode { status: Some(s), .. } if s.is_success()));
    Ok(())
}

#[tokio::test]
async fn auth_endpoints_do_not_send_access_token_header() -> Result<(), Box<dyn std::error::Error>>
{
    let server = MockServer::start().await;

    let create_account_body = json!({
        "intent": "config.auth.createAccount",
        "data": {
            "accountId": "acc",
        }
    });
    let get_auth_code_body = json!({
        "intent": "config.auth.getAuthCode",
        "data": {
            "account": "u",
            "accountType": 1,
            "accessTokenValidity": "7d",
        }
    });
    let get_token_body = json!({
        "intent": "config.auth.getToken",
        "data": {
            "authCode": "code",
            "account": "u",
            "accountType": 1,
        }
    });
    let refresh_token_body = json!({
        "intent": "config.auth.refreshToken",
        "data": {
            "refreshToken": "rt",
        }
    });

    for body in [
        create_account_body,
        get_auth_code_body,
        get_token_body,
        refresh_token_body,
    ] {
        Mock::given(method("POST"))
            .and(path("/v3.0/open/api"))
            .and(header("appid", "APP_ID"))
            .and(header("keyid", "KEY_ID"))
            .and(header("content-type", "application/json"))
            .and(header("lang", "en"))
            .and(header_exists("nonce"))
            .and(header_exists("time"))
            .and(header_exists("sign"))
            .and(HeaderAbsent("accesstoken"))
            .and(body_json(body))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "requestId": "req-1",
                "message": "Success"
            })))
            .expect(1)
            .mount(&server)
            .await;
    }

    let base_url = format!("{}/v3.0/open/api", server.uri()).parse()?;
    let client = Client::builder(Credentials::new("APP_ID", "KEY_ID", "APP_KEY"))
        .base_url(base_url)
        .build()?;

    client
        .auth()
        .create_account(CreateAccountParams::new("acc"))
        .await?;
    client
        .auth()
        .get_auth_code(GetAuthCodeParams::new("u", 1))
        .await?;
    client
        .auth()
        .get_token(GetTokenParams::new("code", "u", 1))
        .await?;
    client
        .auth()
        .refresh_token(RefreshTokenParams::new("rt"))
        .await?;

    Ok(())
}

#[tokio::test]
async fn trait_subscription_serialization_matches_docs() -> Result<(), Box<dyn std::error::Error>> {
    let server = MockServer::start().await;

    let body = json!({
        "intent": "spec.config.trait.subscribe",
        "data": {
            "traits": [
                {
                    "subjectId": "did-1",
                    "codePaths": ["1.2.3"],
                    "attach": "ctx",
                }
            ]
        }
    });

    let _mock = Mock::given(method("POST"))
        .and(path("/v3.0/open/api"))
        .and(header("appid", "APP_ID"))
        .and(header("keyid", "KEY_ID"))
        .and(header("accesstoken", "ACCESS_TOKEN"))
        .and(header("content-type", "application/json"))
        .and(header("lang", "en"))
        .and(header_exists("nonce"))
        .and(header_exists("time"))
        .and(header_exists("sign"))
        .and(body_json(body))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "requestId": "req-1",
            "message": "Success"
        })))
        .expect(1)
        .named("trait subscribe")
        .mount_as_scoped(&server)
        .await;

    let base_url = format!("{}/v3.0/open/api", server.uri()).parse()?;
    let client = Client::builder(Credentials::new("APP_ID", "KEY_ID", "APP_KEY"))
        .base_url(base_url)
        .access_token("ACCESS_TOKEN")
        .build()?;

    client
        .push()
        .subscribe_traits(TraitSubscribeParams::new(vec![
            TraitSubscription::new("did-1", vec!["1.2.3".to_string()]).with_attach("ctx"),
        ]))
        .await?;

    Ok(())
}
