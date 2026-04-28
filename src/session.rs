// Read the saguão session cookie set by passaporte (Authentik).
// We don't validate the JWT client-side — vigia already gates the
// service we're about to link to. We just extract the sub claim
// for display + the user query to crachá.

use serde::Deserialize;
use wasm_bindgen::JsValue;

const COOKIE_NAME: &str = "X-Saguao-Session";

#[derive(Debug, Clone, Deserialize)]
pub struct Claims {
    pub sub: String,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
}

/// Read the session cookie from `document.cookie`. Returns None if
/// no cookie is set or if the JWT can't be base64-decoded.
#[must_use]
pub fn read_session() -> Option<Claims> {
    let document = web_sys::window()?.document()?;
    let html_doc = document.dyn_into::<web_sys::HtmlDocument>().ok()?;
    let cookie_string = html_doc.cookie().ok()?;
    let token = cookie_string
        .split(';')
        .map(str::trim)
        .find_map(|c| c.strip_prefix(&format!("{COOKIE_NAME}=")))?;
    decode_unsafe(token)
}

/// Decode the JWT payload WITHOUT verifying the signature.
/// Safe in this context because we never trust the claims for authz —
/// authz happens at vigia. We only use the sub claim for display +
/// the crachá query.
fn decode_unsafe(token: &str) -> Option<Claims> {
    let mut parts = token.split('.');
    let _header = parts.next()?;
    let payload = parts.next()?;
    let bytes = base64_url_decode(payload).ok()?;
    serde_json::from_slice(&bytes).ok()
}

fn base64_url_decode(s: &str) -> Result<Vec<u8>, ()> {
    // Manual base64-url decode to avoid pulling base64 crate just for this.
    let mut padded = s.replace('-', "+").replace('_', "/");
    while padded.len() % 4 != 0 {
        padded.push('=');
    }
    base64_decode(&padded)
}

fn base64_decode(s: &str) -> Result<Vec<u8>, ()> {
    const TABLE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = Vec::with_capacity(s.len() * 3 / 4);
    let bytes: Vec<u8> = s
        .bytes()
        .filter(|&b| b != b'=')
        .map(|b| {
            TABLE
                .iter()
                .position(|&t| t == b)
                .map(|p| u8::try_from(p).unwrap())
                .ok_or(())
        })
        .collect::<Result<_, _>>()?;

    for chunk in bytes.chunks(4) {
        let mut buf = [0u8; 4];
        for (i, b) in chunk.iter().enumerate() {
            buf[i] = *b;
        }
        let n0 = (buf[0] << 2) | (buf[1] >> 4);
        out.push(n0);
        if chunk.len() > 2 {
            let n1 = (buf[1] << 4) | (buf[2] >> 2);
            out.push(n1);
        }
        if chunk.len() > 3 {
            let n2 = (buf[2] << 6) | buf[3];
            out.push(n2);
        }
    }
    Ok(out)
}

// Imported on demand inside read_session() to keep wasm-bindgen
// happy with the trait in scope.
use wasm_bindgen::JsCast;

// Suppress unused-import warning when compiling on non-wasm targets.
#[allow(dead_code)]
fn _wasm_marker() -> JsValue {
    JsValue::NULL
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_unverified_jwt_payload() {
        // {"sub":"drzln","email":"drzln@protonmail.com","name":"drzln"}
        let payload =
            "eyJzdWIiOiJkcnpsbiIsImVtYWlsIjoiZHJ6bG5AcHJvdG9ubWFpbC5jb20iLCJuYW1lIjoiZHJ6bG4ifQ";
        let token = format!("hdr.{payload}.sig");
        let c = decode_unsafe(&token).unwrap();
        assert_eq!(c.sub, "drzln");
        assert_eq!(c.email.as_deref(), Some("drzln@protonmail.com"));
    }
}
