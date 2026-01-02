use aqara::types::{Credentials, SecretString};

#[test]
fn secret_string_debug_is_redacted() {
    let s = SecretString::new("super-secret");
    let dbg = format!("{s:?}");
    assert!(!dbg.contains("super-secret"));
    assert!(dbg.contains("REDACTED"));
}

#[test]
fn credentials_debug_redacts_app_key() {
    let c = Credentials::new("APP_ID", "KEY_ID", "APP_KEY");
    let dbg = format!("{c:?}");
    assert!(dbg.contains("APP_ID"));
    assert!(dbg.contains("KEY_ID"));
    assert!(!dbg.contains("APP_KEY"));
    assert!(dbg.contains("REDACTED"));
}
