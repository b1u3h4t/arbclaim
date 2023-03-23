use std::{str::FromStr, sync::Arc};

use ethers::prelude::{
    k256::ecdsa::SigningKey, Address, Http, LocalWallet, NonceManagerMiddleware, Provider, Signer,
    SignerMiddleware, Wallet,
};

use ethers::providers::Middleware;

pub type ProviderType =
    Arc<NonceManagerMiddleware<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>>;

#[derive(Debug)]
pub struct Account {
    pub client: ProviderType,
    pub sender: Address,
    pub private_key: String,
}

impl Account {
    pub async fn new(private_key: &str, rpc: &str) -> Self {
        let mut private_key = private_key.to_string();
        if private_key.starts_with("0x") {
            private_key = private_key.replace("0x", "");
        }
        let provider = Arc::new(Provider::<Http>::try_from(rpc).unwrap());
        let chainid = provider.get_chainid().await.unwrap();
        let wallet = LocalWallet::from_str(&private_key)
            .unwrap()
            .with_chain_id(chainid.as_u64());
        let sender = wallet.address();
        let client = Arc::new(NonceManagerMiddleware::new(
            SignerMiddleware::new(provider, wallet),
            sender,
        ));
        Self {
            client,
            sender,
            private_key,
        }
    }
}
