use crate::shared::contracts::ERC20;
use crate::shared::contracts::ERC20::ERC20Instance;
use crate::shared::execute_call::execute_call;
use crate::shared::signed_provider::SignedProvider;
use alloy::primitives::{Address, TxHash, U256};
use alloy::providers::WalletProvider;
use alloy::pubsub::PubSubFrontend;
use anyhow::{bail, Result};

pub struct WalletAndAmount {
    pub address: Address,
    pub to_check_amount: U256,
}

#[derive(Clone)]
pub struct Erc20Service {
    provider: SignedProvider,
}

impl Erc20Service {
    pub fn new(provider: SignedProvider) -> Self {
        Self { provider }
    }

    pub async fn check_wallets_allowances(
        &self,
        token_address: Address,
        wallets: Vec<WalletAndAmount>,
        spender: Address,
    ) -> Result<()> {
        let contract_instance = ERC20::new(token_address, self.provider.clone());

        for wallet in wallets {
            let allowance: U256 = self
                .fetch_allowance(contract_instance.clone(), wallet.address, spender)
                .await?;

            if allowance < wallet.to_check_amount {
                bail!(
                    "Wallet {} has insufficient allowance: needed {}, but has {}",
                    wallet.address,
                    wallet.to_check_amount,
                    allowance
                );
            }
        }

        Ok(())
    }

    pub async fn check_signer_allowance_or_approve(
        &self,
        token_address: Address,
        spender: Address,
        target_amount: U256,
    ) -> Result<Option<TxHash>> {
        let contract_instance = ERC20::new(token_address, self.provider.clone());

        let signer_address = self.provider.default_signer_address();

        let allowance: U256 = self
            .fetch_allowance(contract_instance.clone(), signer_address, spender)
            .await?;

        if allowance < target_amount {
            return Ok(Some(
                self.approve_spent_amount(contract_instance.clone(), spender, target_amount)
                    .await?,
            ));
        }

        Ok(None)
    }

    pub async fn fetch_balance(&self, token_address: Address, owner: Address) -> Result<U256> {
        let contract_instance = ERC20::new(token_address, self.provider.clone());

        let res = contract_instance.balanceOf(owner).call().await?._0;

        Ok(res)
    }

    async fn fetch_allowance(
        &self,
        contract: ERC20Instance<PubSubFrontend, SignedProvider>,
        owner: Address,
        spender: Address,
    ) -> Result<U256> {
        let res = contract.allowance(owner, spender).call().await?._0;

        Ok(res)
    }

    async fn approve_spent_amount(
        &self,
        contract: ERC20Instance<PubSubFrontend, SignedProvider>,
        spender: Address,
        amount: U256,
    ) -> Result<TxHash> {
        let template = contract.approve(spender, amount);

        Ok(execute_call(template, "approve_spent_amount").await?)
    }
}
