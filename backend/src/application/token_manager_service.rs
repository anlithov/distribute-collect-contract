use crate::shared::contracts::TokenManager::TokenManagerInstance;
use crate::shared::execute_call::execute_call;
use crate::shared::signed_provider::SignedProvider;
use alloy::primitives::{Address, TxHash, U256};
use alloy::pubsub::PubSubFrontend;
use anyhow::Result;

#[derive(Clone)]
pub struct TokenManagerService {
    contract: TokenManagerInstance<PubSubFrontend, SignedProvider>,
}

impl TokenManagerService {
    pub fn new(contract: TokenManagerInstance<PubSubFrontend, SignedProvider>) -> Self {
        Self { contract }
    }

    pub async fn distribute_native_tokens(
        &self,
        receivers: Vec<Address>,
        proportions: Vec<U256>,
        total_amount: U256,
    ) -> Result<TxHash> {
        let template = self
            .contract
            .distributeNativeTokens(receivers, proportions, total_amount)
            .value(total_amount);

        Ok(execute_call(template, "distribute_native_tokens").await?)
    }

    pub async fn distribute_erc20_tokens(
        &self,
        token_address: Address,
        receivers: Vec<Address>,
        proportions: Vec<U256>,
        total_amount: U256,
    ) -> Result<TxHash> {
        let template = self.contract.distributeERC20Tokens(
            token_address,
            receivers,
            proportions,
            total_amount,
        );

        Ok(execute_call(template, "distribute_erc20_tokens").await?)
    }

    pub async fn collect_erc20_tokens(
        &self,
        token_address: Address,
        froms: Vec<Address>,
        scaled_percents: Vec<U256>,
    ) -> Result<TxHash> {
        let template = self
            .contract
            .collectERC20Tokens(token_address, froms, scaled_percents);

        Ok(execute_call(template, "collect_erc20_tokens").await?)
    }

    pub fn get_token_manager_address(&self) -> Address {
        self.contract.address().clone()
    }
}
