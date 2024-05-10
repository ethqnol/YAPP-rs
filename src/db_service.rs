use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use firebase::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Students {
    #[serde(alias = "_firestore_id")]
    id: Option<String>,
    first_name: String,
    last_name: String,
    email: String,
}
