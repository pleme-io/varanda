// Model types — mirror cracha-api's REST response shapes for
// client-side deserialization. Keeping a local copy avoids pulling
// cracha-* crates (which have kube-rs / sea-orm deps) into the
// WASM build.

use serde::{Deserialize, Serialize};

/// One service tile rendered in the portal.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccessibleService {
    pub slug: String,
    pub display_name: String,
    pub cluster: String,
    pub location: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub hostname: String,
}

/// Role flag from cracha. Drives whether the admin panel renders.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    User,
}

/// `GET /me` response — full user manifest. cracha computes this
/// from the JWT-derived identity + declarative AccessPolicy CRDs +
/// DB-stored per-user grants.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MeResponse {
    pub user_id: String,
    pub email: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub role: Role,
    pub services: Vec<AccessibleService>,
}

/// `GET /admin/audit` response — a tail of state-mutation events.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuditEvent {
    pub id: String,
    pub ts: String,
    pub actor_user_id: Option<String>,
    pub actor_email: String,
    pub action: String,
    pub target_kind: String,
    pub target_id: String,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuditResponse {
    pub events: Vec<AuditEvent>,
}

/// `POST /admin/grants` request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddGrantRequest {
    pub user_id: String,
    pub service: String,
    pub verb: String,
    pub expires_at: Option<String>,
    pub note: Option<String>,
}

/// `DELETE /admin/grants` request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeGrantRequest {
    pub user_id: String,
    pub service: String,
    pub verb: String,
}
