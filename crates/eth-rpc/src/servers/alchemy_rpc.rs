use std::sync::Arc;

use jsonrpsee::core::{async_trait, RpcResult as Result};
use kakarot_rpc_core::client::KakarotClient;
use kakarot_rpc_core::models::balance::TokenBalances;
use reth_primitives::Address;
use starknet::providers::Provider;

use crate::api::alchemy_api::AlchemyApiServer;

/// The RPC module for the Ethereum protocol required by Kakarot.
pub struct AlchemyRpc<P: Provider + Send + Sync + 'static> {
    pub kakarot_client: Arc<KakarotClient<P>>,
}

impl<P: Provider + Send + Sync + 'static> AlchemyRpc<P> {
    pub fn new(kakarot_client: Arc<KakarotClient<P>>) -> Self {
        Self { kakarot_client }
    }
}

#[async_trait]
impl<P: Provider + Send + Sync + 'static> AlchemyApiServer for AlchemyRpc<P> {
    async fn token_balances(&self, address: Address, contract_addresses: Vec<Address>) -> Result<TokenBalances> {
        let token_balances = self.kakarot_client.token_balances(address, contract_addresses).await?;
        Ok(token_balances)
    }
}
