use crate::shared::signed_provider::SignedProvider;
use alloy::contract::SolCallBuilder;
use alloy::primitives::TxHash;
use alloy::pubsub::PubSubFrontend;
use alloy::sol_types::SolCall;
use anyhow::Result;

pub async fn execute_call<T>(
    call: SolCallBuilder<PubSubFrontend, &SignedProvider, T>,
    service_name: &str,
) -> Result<TxHash>
where
    T: SolCall,
{
    let pending_tx = call.send().await?;

    println!(
        "{}. Pending transaction... {}",
        service_name,
        pending_tx.tx_hash()
    );

    let receipt = pending_tx.get_receipt().await?;

    println!(
        "{}. Transaction successful with hash: {:?}",
        service_name, receipt.transaction_hash
    );

    Ok(receipt.transaction_hash)
}
