use crate::application::action_service::ActionService;
use crate::application::erc20_service::Erc20Service;
use crate::application::token_manager_service::TokenManagerService;
use alloy::primitives::Address;
use alloy::pubsub::PubSubFrontend;
use anyhow::Result;
use axum::response::IntoResponse;
use axum::Router;
use serde::Serialize;
use shared::contracts::TokenManager;
use shared::contracts::TokenManager::TokenManagerInstance;
use shared::signed_provider::{SignedProvider, Web3Provider};
use std::str::FromStr;
use std::sync::Arc;

mod api;
mod application;
mod shared;
mod ui;

#[derive(Clone)]
pub struct AppState {
    contract: Arc<TokenManagerInstance<PubSubFrontend, SignedProvider>>,
}
#[derive(Debug, Serialize)]
pub struct AppResponse {
    pub tx_hash_approve: Option<String>,
    pub tx_hash_distribute: Option<String>,
    pub error: Option<String>,
}

impl AppState {
    pub fn new(contract: Arc<TokenManagerInstance<PubSubFrontend, SignedProvider>>) -> Self {
        Self { contract }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("->> Starting application!");
    dotenvy::dotenv()?;

    let contract_address_str = dotenvy::var("TOKEN_MANAGER_ADDRESS")?;

    let contract_address = Address::from_str(&contract_address_str)?;
    let provider = Web3Provider::prepare_ws_signed().await?;

    let token_manager_instance = TokenManager::new(contract_address, provider.clone());

    let token_manager_service = TokenManagerService::new(token_manager_instance);
    let erc20_service = Erc20Service::new(provider.clone());

    let routes_distribute = api::routes_distribute::routes(ActionService::new(
        erc20_service.clone(),
        token_manager_service.clone(),
    ));
    let routes_collect =
        api::routes_collect::routes(ActionService::new(erc20_service, token_manager_service));

    // build our application with a route
    let routes = Router::new()
        .merge(routes_distribute)
        .merge(routes_collect)
        .merge(ui::routes_root());

    let port = dotenvy::var("PORT").unwrap_or("5000".to_string());

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await?;

    println!(
        "->> Application is running on {:?}!",
        listener.local_addr()?
    );

    axum::serve(listener, routes).await?;

    Ok(())
}
