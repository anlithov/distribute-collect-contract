use crate::api::routes_collect::CollectErc20Payload;
use crate::api::routes_distribute::{DistributeBasePayload, DistributeErc20Payload};
use crate::application::erc20_service::{Erc20Service, WalletAndAmount};
use crate::application::token_manager_service::TokenManagerService;
use crate::AppResponse;
use alloy::primitives::{Address, U256};
use anyhow::Result;
use std::str::FromStr;

#[derive(Clone)]
pub struct ActionService {
    pub erc20_service: Erc20Service,
    pub token_manager_service: TokenManagerService,
}

impl ActionService {
    pub fn new(erc20_service: Erc20Service, token_manager_service: TokenManagerService) -> Self {
        Self {
            erc20_service,
            token_manager_service,
        }
    }
}

impl ActionService {
    pub async fn distribute_native_tokens(
        &self,
        payload: DistributeBasePayload,
    ) -> Result<AppResponse> {
        let (receivers, proportions, amount) = self.transform_args_to_alloy(payload);

        let tx_hash = self
            .token_manager_service
            .distribute_native_tokens(receivers, proportions, amount)
            .await?;

        Ok(AppResponse {
            tx_hash_distribute: Some(tx_hash.to_string()),
            tx_hash_approve: None,
            error: None,
        })
    }

    pub async fn distribute_erc20_tokens(
        &self,
        payload: DistributeErc20Payload,
    ) -> Result<AppResponse> {
        let token_address = payload.token_address.parse::<Address>()?.clone();

        let (receivers, proportions, amount) = self.transform_args_to_alloy(payload.base);

        let token_manager_address = self.token_manager_service.get_token_manager_address();

        let approve_hash = self
            .erc20_service
            .check_signer_allowance_or_approve(token_address.clone(), token_manager_address, amount)
            .await?;

        let tx_hash = self
            .token_manager_service
            .distribute_erc20_tokens(token_address.clone(), receivers, proportions, amount)
            .await?;

        Ok(AppResponse {
            tx_hash_distribute: Some(tx_hash.to_string()),
            tx_hash_approve: Some(approve_hash.unwrap().to_string()),
            error: None,
        })
    }

    fn transform_args_to_alloy(
        &self,
        payload: DistributeBasePayload,
    ) -> (Vec<Address>, Vec<U256>, U256) {
        let mut receivers: Vec<Address> = vec![];
        let mut proportions: Vec<U256> = vec![];

        for set in payload.receivers_with_proportions {
            receivers.push(set.receiver.parse::<Address>().unwrap());
            proportions.push(set.proportion.parse::<U256>().unwrap())
        }

        let amount = U256::from_str(&payload.amount).unwrap();

        (receivers, proportions, amount)
    }

    pub async fn collect_erc20_tokens(&self, payload: CollectErc20Payload) -> Result<AppResponse> {
        let token_address = payload.token_address.parse::<Address>()?.clone();

        let mut froms: Vec<Address> = vec![];
        let mut scaled_percents: Vec<U256> = vec![];

        for set in payload.sets {
            froms.push(set.from.parse::<Address>()?);
            scaled_percents.push(set.scaled_percent.parse::<U256>()?)
        }

        let mut wallets_and_balances_to_be_sent: Vec<WalletAndAmount> = vec![];

        for (pos, _) in froms.iter().enumerate() {
            let wallet_address = froms[pos];
            let scaled_percent = scaled_percents[pos];

            let balance = self
                .erc20_service
                .fetch_balance(token_address.clone(), wallet_address)
                .await?;
            // In contract  uint256 public constant PERCENT_PRECISION = 1_000_000;
            let to_check_amount = balance * scaled_percent / U256::from(1_000_000);

            wallets_and_balances_to_be_sent.push(WalletAndAmount {
                address: wallet_address,
                to_check_amount,
            })
        }

        let token_manager_address = self.token_manager_service.get_token_manager_address();

        self.erc20_service
            .check_wallets_allowances(
                token_address.clone(),
                wallets_and_balances_to_be_sent,
                token_manager_address,
            )
            .await?;

        let tx_hash = self
            .token_manager_service
            .collect_erc20_tokens(token_address.clone(), froms, scaled_percents)
            .await?;

        Ok(AppResponse {
            tx_hash_distribute: Some(tx_hash.to_string()),
            tx_hash_approve: None,
            error: None,
        })
    }
}
