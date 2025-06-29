use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{FixedBytes, U256},
    providers::{DynProvider, Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    sol,
};
use anyhow::{anyhow, Result};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct EthereumClient {
    provider: DynProvider,
    signer: PrivateKeySigner,
    instance: Auditability::AuditabilityInstance<(), DynProvider>,
}

impl EthereumClient {
    pub fn new(url: String, contract: String, pk: String) -> Result<Self> {
        let signer: PrivateKeySigner = pk.parse()?;
        let wallet = EthereumWallet::from(signer.clone());
        let url_parsed = url.parse()?;
        let provider = ProviderBuilder::new().wallet(wallet).on_http(url_parsed);
        let provider = DynProvider::new(provider);
        let contract = contract.parse()?;
        let instance = Auditability::new(contract, provider.clone());

        Ok(Self { provider, signer, instance })
    }

    pub async fn nonce(&self) -> Result<u64> {
        let address = self.signer.address();
        let nonce = self.provider.get_transaction_count(address).await?;
        Ok(nonce)
    }

    pub async fn remove_tx(&self, nonce: u64) -> Result<FixedBytes<32>> {
        let address = self.signer.address();

        let tx = TransactionRequest::default()
            .with_to(address)
            .with_nonce(nonce)
            .with_value(U256::ZERO)
            .with_gas_limit(21_000)
            .with_max_priority_fee_per_gas(1_000_000_000)
            .with_max_fee_per_gas(20_000_000_000);

        let tx = self.provider.send_transaction(tx).await?;
        let receipt = tx.get_receipt().await?;

        Ok(receipt.transaction_hash)
    }
    pub async fn store(&self, nonce: u64, index: &String, hash: FixedBytes<32>) -> Result<FixedBytes<32>> {
        let mut attempt = 0;

        loop {
            let result = {
                let call = self.instance.store(index.clone(), hash).nonce(nonce);
                call.send().await
            };

            if let Ok(tx) = result {
                return Ok(tx.tx_hash().clone());
            }

            attempt += 1;
            if attempt >= 3 {
                return Err(anyhow!("failed to send tx: {}", result.unwrap_err()));
            }

            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    }

    pub async fn exists(&self, index: String) -> Result<bool> {
        let result = self.instance.exists(index).call().await?;
        Ok(result._0)
    }

    pub async fn hash(&self, index: String) -> Result<FixedBytes<32>> {
        let result = self.instance.hash(index).call().await?;
        Ok(result._0)
    }

    pub async fn wait_tx(&self, tx_hash: FixedBytes<32>) -> Result<FixedBytes<32>> {
        let mut interval = tokio::time::interval(Duration::from_millis(500));
        loop {
            interval.tick().await;

            match self.provider.get_transaction_receipt(tx_hash).await {
                Ok(Some(receipt)) => return Ok(receipt.transaction_hash),
                Ok(None) => continue,
                Err(err) => return Err(anyhow!("failed to get transaction receipt: {err}")),
            }
        }
    }
}

sol! {
    #[sol(rpc)]
    contract Auditability {
        event IndexStored(string indexed index, bytes32 hash);
        function store(string index, bytes32 root) external;
        function proof(string index, bytes32 root) external view returns (bool);
        function hash(string index) external view returns (bytes32);
        function exists(string index) external view returns (bool);
        function owner() external view returns (address);
    }
}
