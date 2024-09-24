use alloy::sol;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug)]
    TokenManager,
    "../foundry/out/TokenManager.sol/TokenManager.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug)]
    ERC20,
    "../foundry/out/ERC20.sol/ERC20.json"
);
