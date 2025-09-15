use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::Uuid;
use chrono::{Utc, Duration};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Grants {
    #[serde(skip_serializing_if = "Option::is_none")]
    room: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub: Option<String>,
    jti: String,
    iat: i64,
    exp: i64,
    nbf: i64,
    identity: String,
    grants: Grants,
}

#[tauri::command]
fn generate_token(
    room: String,
    identity: Option<String>,
    ttl_minutes: Option<i64>,
) -> Result<String, String> {
    // Load environment variables from .env (during dev)
    dotenv::dotenv().ok();

    let api_key = env::var("LIVEKIT_API_KEY")
        .map_err(|_| "LIVEKIT_API_KEY not set".to_string())?;
    let api_secret = env::var("LIVEKIT_API_SECRET")
        .map_err(|_| "LIVEKIT_API_SECRET not set".to_string())?;

    // If no identity passed from frontend, generate random one
    let identity = identity.unwrap_or_else(|| format!("user-{}", Uuid::new_v4().to_simple()));

    // Default token lifetime = 60 minutes
    let ttl = ttl_minutes.unwrap_or(60);
    let now = Utc::now();
    let iat = now.timestamp();
    let exp = (now + Duration::minutes(ttl)).timestamp();
    let jti = Uuid::new_v4().to_string();

    let claims = Claims {
        iss: api_key.clone(),
        sub: None,
        jti,
        iat,
        exp,
        nbf: iat,
        identity,
        grants: Grants { room: Some(room) },
    };

    let header = Header::default();

    encode(&header, &claims, &EncodingKey::from_secret(api_secret.as_bytes()))
        .map_err(|e| format!("JWT encode error: {}", e))
}

pub fn run() {
    tauri::Builder::new() // ✅ Tauri 2.x API
        .invoke_handler(tauri::generate_handler![generate_token])
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
