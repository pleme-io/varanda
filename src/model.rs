// Model types — mirror cracha-core::AccessibleService for client-side
// deserialization. Keeping a local copy avoids pulling cracha-core
// (which has kube-rs deps) into a WASM build.

use serde::{Deserialize, Serialize};

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
