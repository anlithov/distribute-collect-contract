use crate::application::action_service::ActionService;
use crate::AppResponse;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;

pub fn routes(dc: ActionService) -> Router {
    Router::new()
        .route("/distribute/native", post(distribute_native_tokens))
        .route("/distribute/erc20", post(distribute_erc20_tokens))
        .with_state(dc)
}

#[derive(Debug, Deserialize)]
pub struct ReceiversWithProportions {
    pub receiver: String,
    pub proportion: String,
}

#[derive(Debug, Deserialize)]
pub struct DistributeBasePayload {
    pub receivers_with_proportions: Vec<ReceiversWithProportions>,
    pub amount: String,
}

async fn distribute_native_tokens(
    State(dc): State<ActionService>,
    Json(payload): Json<DistributeBasePayload>,
) -> Json<AppResponse> {
    println!("->> distribute_erc20_tokens. Params: {:?}", payload);
    /*    Json(AppResponse {
        tx_hash_approve: None,
        tx_hash_distribute: None,
        error: None,
    })*/
    match dc.distribute_native_tokens(payload).await {
        Ok(res) => Json(res),
        Err(e) => Json(AppResponse {
            tx_hash_approve: None,
            tx_hash_distribute: None,
            error: Some(format!("Error: {}", e)),
        }),
    }
}

#[derive(Debug, Deserialize)]
pub struct DistributeErc20Payload {
    pub base: DistributeBasePayload,
    pub token_address: String,
}

async fn distribute_erc20_tokens(
    State(dc): State<ActionService>,
    Json(payload): Json<DistributeErc20Payload>,
) -> Json<AppResponse> {
    println!("->> distribute_erc20_tokens. Params: {:?}", payload);
    /* Json(AppResponse {
        tx_hash_approve: None,
        tx_hash_distribute: None,
        error: None,
    })*/
    match dc.distribute_erc20_tokens(payload).await {
        Ok(res) => Json(res),
        Err(e) => Json(AppResponse {
            tx_hash_approve: None,
            tx_hash_distribute: None,
            error: Some(format!("Error: {}", e)),
        }),
    }
}
