use alloy::network::{Ethereum, EthereumWallet};
use alloy::providers::fillers::{FillProvider, JoinFill, RecommendedFiller, WalletFiller};
use alloy::providers::{ProviderBuilder, RootProvider, WsConnect};
use alloy::pubsub::PubSubFrontend;
use alloy::signers::local::PrivateKeySigner;
use anyhow::Result;

pub type SignedProvider = FillProvider<
    JoinFill<RecommendedFiller, WalletFiller<EthereumWallet>>,
    RootProvider<PubSubFrontend>,
    PubSubFrontend,
    Ethereum,
>;

pub struct Web3Provider {}
impl Web3Provider {
    pub async fn prepare_ws_signed() -> Result<SignedProvider> {
        let rpc_url = dotenvy::var("RPC_URL")?;

        let ws = WsConnect::new(rpc_url);

        // Wallet
        let private_key = dotenvy::var("PRIVATE_KEY")?;

        let signer: PrivateKeySigner = private_key.parse().expect("Incorrect or empty private key");

        let wallet = EthereumWallet::from(signer);

        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_ws(ws)
            .await?;

        println!("->> Signed provider is ready!");

        Ok(provider)
    }
}
