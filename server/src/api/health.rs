// © Copyright 2024 the cozyauth developers
// SPDX-License-Identifier: GPL-3.0-or-later

use axum::{routing::get, Router};

pub(crate) fn router() -> Router {
    Router::new().route("/health", get(|| async { "OK" }))
}