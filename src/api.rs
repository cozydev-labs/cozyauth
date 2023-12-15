use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;
use webauthn_rs_proto::{PublicKeyCredentialCreationOptions, RegisterPublicKeyCredential};

use crate::{db, passkeys, types::User};

pub(crate) async fn start_passkey_registration(
    State(pool): State<PgPool>,
    Json(user): Json<User>,
) -> Json<PublicKeyCredentialCreationOptions> {
    let mut conn = pool.acquire().await.unwrap();
    let relying_party = db::get_relying_party(&mut conn).unwrap();
    let (ccr, skr) = passkeys::start_registration(relying_party, user.clone());
    db::new_passkey_registration(&mut conn, user.id, skr)
        .await
        .unwrap();
    Json(ccr.public_key)
}

pub(crate) async fn finish_passkey_registration(
    State(pool): State<PgPool>,
    Path(user_id): Path<Uuid>,
    Json(reg): Json<RegisterPublicKeyCredential>,
) -> Json<Value> {
    let mut conn = pool.acquire().await.unwrap();
    let relying_party = db::get_relying_party(&mut conn).unwrap();
    let state = db::get_passkey_registration(&mut conn, user_id)
        .await
        .unwrap();
    let passkey = passkeys::finish_registration(relying_party, reg, state);
    let message = format!("Passkey {} registered ✅", passkey.cred_id());
    Json(json!({ "ok": message }))
}