use base64::encode;

pub fn create_auth_header(auth_string: &str) -> String {
    let auth_payload = encode(auth_string);
    format!("Basic {}", auth_payload)
}
