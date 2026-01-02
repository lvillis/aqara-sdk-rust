use serde_json::{Map, Value};

pub(crate) fn redact_json(mut value: Value) -> Value {
    redact_value_in_place(&mut value);
    value
}

pub(crate) fn snippet_from_bytes(bytes: &[u8], max_len: usize) -> String {
    if max_len == 0 {
        return String::new();
    }

    let as_json = serde_json::from_slice::<Value>(bytes).ok();
    let mut s = match as_json {
        Some(v) => redact_json(v).to_string(),
        None => String::from_utf8_lossy(bytes).to_string(),
    };

    if s.len() > max_len {
        s.truncate(max_len);
    }
    s
}

fn redact_value_in_place(value: &mut Value) {
    match value {
        Value::Object(map) => redact_map_in_place(map),
        Value::Array(arr) => {
            for v in arr {
                redact_value_in_place(v);
            }
        }
        _ => {}
    }
}

fn redact_map_in_place(map: &mut Map<String, Value>) {
    for (k, v) in map.iter_mut() {
        if is_sensitive_key(k) {
            *v = Value::String("[REDACTED]".to_string());
            continue;
        }
        redact_value_in_place(v);
    }
}

fn is_sensitive_key(key: &str) -> bool {
    let key = key.trim().to_ascii_lowercase();
    matches!(
        key.as_str(),
        "accesstoken"
            | "access_token"
            | "access-token"
            | "token"
            | "refresh_token"
            | "refreshtoken"
            | "appkey"
            | "app_key"
            | "password"
            | "secret"
    ) || key.contains("token")
}
