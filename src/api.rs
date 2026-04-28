// crachá REST client — fetches the user's portal manifest.

use crate::model::AccessibleService;
use gloo_net::http::Request;

/// crachá fleet endpoint. Configurable at build time via the
/// `CRACHA_API_URL` env var; defaults to the canonical fleet host.
const DEFAULT_CRACHA_API: &str = "https://cracha.quero.cloud";

#[must_use]
pub fn cracha_url() -> String {
    option_env!("CRACHA_API_URL")
        .unwrap_or(DEFAULT_CRACHA_API)
        .to_string()
}

/// Fetch the user's accessible services from crachá.
pub async fn accessible_services(user: &str) -> Result<Vec<AccessibleService>, String> {
    let url = format!(
        "{}/accessible-services?user={}",
        cracha_url(),
        urlencoding_encode(user)
    );
    let resp = Request::get(&url)
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.ok() {
        return Err(format!("crachá responded {}", resp.status()));
    }
    resp.json::<Vec<AccessibleService>>()
        .await
        .map_err(|e| e.to_string())
}

fn urlencoding_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            other => out.push_str(&format!("%{other:02X}")),
        }
    }
    out
}
