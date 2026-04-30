// crachá REST client — fetches the user's portal manifest +
// admin-side CRUD endpoints.

use crate::model::{
    AccessibleService, AddGrantRequest, AuditResponse, MeResponse, RevokeGrantRequest,
};
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

/// Sentinel for "no session cookie / 401 from cracha". The caller
/// translates this to "render the sign-in screen."
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiError {
    /// 401 from cracha — JWT missing or invalid; user must re-auth.
    Unauthorized,
    /// 403 — JWT is fine but the action requires admin role.
    Forbidden,
    /// Anything else (network, 5xx, parse failure).
    Other(String),
}

impl ApiError {
    pub fn message(&self) -> String {
        match self {
            ApiError::Unauthorized => "session required".into(),
            ApiError::Forbidden => "admin role required".into(),
            ApiError::Other(s) => s.clone(),
        }
    }
}

/// `GET /me` — authoritative user manifest. The session cookie
/// (X-Saguao-Session, set by passaporte) is forwarded automatically
/// via `credentials: include`.
pub async fn fetch_me() -> Result<MeResponse, ApiError> {
    let url = format!("{}/me", cracha_url());
    let resp = Request::get(&url)
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| ApiError::Other(e.to_string()))?;
    match resp.status() {
        200 => resp
            .json::<MeResponse>()
            .await
            .map_err(|e| ApiError::Other(e.to_string())),
        401 => Err(ApiError::Unauthorized),
        403 => Err(ApiError::Forbidden),
        s => Err(ApiError::Other(format!("crachá responded {s}"))),
    }
}

/// `POST /admin/grants` — admin-only.
pub async fn add_grant(req: &AddGrantRequest) -> Result<(), ApiError> {
    let url = format!("{}/admin/grants", cracha_url());
    let resp = Request::post(&url)
        .credentials(web_sys::RequestCredentials::Include)
        .header("Content-Type", "application/json")
        .json(req)
        .map_err(|e| ApiError::Other(e.to_string()))?
        .send()
        .await
        .map_err(|e| ApiError::Other(e.to_string()))?;
    match resp.status() {
        200 | 201 | 204 => Ok(()),
        401 => Err(ApiError::Unauthorized),
        403 => Err(ApiError::Forbidden),
        s => Err(ApiError::Other(format!("crachá responded {s}"))),
    }
}

/// `DELETE /admin/grants` — admin-only.
pub async fn revoke_grant(req: &RevokeGrantRequest) -> Result<(), ApiError> {
    let url = format!("{}/admin/grants", cracha_url());
    let resp = Request::delete(&url)
        .credentials(web_sys::RequestCredentials::Include)
        .header("Content-Type", "application/json")
        .json(req)
        .map_err(|e| ApiError::Other(e.to_string()))?
        .send()
        .await
        .map_err(|e| ApiError::Other(e.to_string()))?;
    match resp.status() {
        200 | 204 => Ok(()),
        401 => Err(ApiError::Unauthorized),
        403 => Err(ApiError::Forbidden),
        404 => Err(ApiError::Other("no matching grant".into())),
        s => Err(ApiError::Other(format!("crachá responded {s}"))),
    }
}

/// `GET /admin/audit?limit=N` — admin-only. Recent state-mutation events.
pub async fn fetch_audit(limit: u32) -> Result<AuditResponse, ApiError> {
    let url = format!("{}/admin/audit?limit={limit}", cracha_url());
    let resp = Request::get(&url)
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| ApiError::Other(e.to_string()))?;
    match resp.status() {
        200 => resp
            .json::<AuditResponse>()
            .await
            .map_err(|e| ApiError::Other(e.to_string())),
        401 => Err(ApiError::Unauthorized),
        403 => Err(ApiError::Forbidden),
        s => Err(ApiError::Other(format!("crachá responded {s}"))),
    }
}

/// Legacy helper kept for backwards compatibility while the lib is
/// still consumed by older callers. New code calls `fetch_me`.
#[deprecated(note = "Use fetch_me() — returns identity + services + role")]
#[allow(dead_code)]
pub async fn accessible_services(_user: &str) -> Result<Vec<AccessibleService>, String> {
    fetch_me()
        .await
        .map(|m| m.services)
        .map_err(|e| e.message())
}
