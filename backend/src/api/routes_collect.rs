use crate::application::action_service::ActionService;
use crate::AppResponse;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;

pub fn routes(dc: ActionService) -> Router {
    Router::new()
        .route("/collect/erc20", post(collect_erc20_tokens))
        .with_state(dc)
}

#[derive(Debug, Deserialize)]
pub struct FromWalletWithPercent {
    pub from: String,
    pub scaled_percent: String,
}

#[derive(Debug, Deserialize)]
pub struct CollectErc20Payload {
    pub sets: Vec<FromWalletWithPercent>,
    pub token_address: String,
}

async fn collect_erc20_tokens(
    State(dc): State<ActionService>,
    Json(payload): Json<CollectErc20Payload>,
) -> Json<AppResponse> {
    println!("->> collect_erc20_tokens. Params: {:?}", payload);

    match dc.collect_erc20_tokens(payload).await {
        Ok(res) => Json(res),
        Err(e) => Json(AppResponse {
            tx_hash_approve: None,
            tx_hash_distribute: None,
            error: Some(format!("Error: {}", e)),
        }),
    }
}
