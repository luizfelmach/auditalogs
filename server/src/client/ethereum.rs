use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{Address, FixedBytes, U256},
    providers::{DynProvider, Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    sol,
};
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EthereumClientError {
    #[error("Invalid private key: {0}")]
    InvalidPrivateKey(String),
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    #[error("Invalid contract address: {0}")]
    InvalidContractAddress(String),
    #[error("Transaction send failed: {0}")]
    TransactionSendError(String),
    #[error("Failed to fetch transaction receipt: {0}")]
    ReceiptFetchError(String),
    #[error("Failed to parse transaction hash: {0}")]
    TransactionReceiptNotFound(String),
}

#[derive(Debug, Clone)]
pub struct EthereumClient {
    provider: DynProvider,
    contract: Address,
    signer: PrivateKeySigner,
}

impl EthereumClient {
    pub fn new(url: String, contract: String, pk: String) -> Result<Self, EthereumClientError> {
        let signer: PrivateKeySigner = pk
            .parse()
            .map_err(|_| EthereumClientError::InvalidPrivateKey(pk.clone()))?;
        let wallet = EthereumWallet::from(signer.clone());

        let url_parsed = url
            .parse()
            .map_err(|_| EthereumClientError::InvalidUrl(url.clone()))?;
        let provider = ProviderBuilder::new().wallet(wallet).on_http(url_parsed);
        let provider = DynProvider::new(provider);

        let contract = contract
            .parse()
            .map_err(|_| EthereumClientError::InvalidContractAddress(contract.clone()))?;

        Ok(Self {
            provider,
            contract,
            signer,
        })
    }

    pub async fn nonce(&self) -> Result<u64, EthereumClientError> {
        let address = self.signer.address();
        self.provider
            .get_transaction_count(address)
            .await
            .map_err(|e| EthereumClientError::ReceiptFetchError(e.to_string()))
    }

    pub async fn remove_tx(&self, tx_nonce: u64) -> Result<FixedBytes<32>, EthereumClientError> {
        let address = self.signer.address();

        let tx = TransactionRequest::default()
            .with_to(address)
            .with_nonce(tx_nonce)
            .with_value(U256::ZERO)
            .with_gas_limit(21_000)
            .with_max_priority_fee_per_gas(1_000_000_000)
            .with_max_fee_per_gas(20_000_000_000);

        let pending_tx = self
            .provider
            .send_transaction(tx)
            .await
            .map_err(|e| EthereumClientError::TransactionSendError(e.to_string()))?;

        let receipt = pending_tx
            .get_receipt()
            .await
            .map_err(|e| EthereumClientError::ReceiptFetchError(e.to_string()))?;

        Ok(receipt.transaction_hash)
    }
    pub async fn send_tx(
        &self,
        nonce: u64,
        index: &str,
        hash: FixedBytes<32>,
        attempts: u32,
    ) -> Result<FixedBytes<32>, EthereumClientError> {
        let contract = self.contract();

        let mut current_attempt = 0;
        loop {
            let result = contract
                .store(index.to_owned(), hash)
                .nonce(nonce)
                .send()
                .await;
            match result {
                Ok(tx) => {
                    let tx_hash = tx.tx_hash().clone();
                    return Ok(tx_hash);
                }
                Err(e) => {
                    current_attempt += 1;
                    if current_attempt >= attempts {
                        return Err(EthereumClientError::TransactionSendError(e.to_string()));
                    }
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }

    pub async fn wait_for_tx(
        &self,
        tx_hash: FixedBytes<32>,
    ) -> Result<FixedBytes<32>, EthereumClientError> {
        let mut interval = tokio::time::interval(Duration::from_millis(500));

        loop {
            interval.tick().await;

            match self.provider.get_transaction_receipt(tx_hash).await {
                Ok(Some(receipt)) => return Ok(receipt.transaction_hash),
                Ok(None) => continue,
                Err(e) => {
                    return Err(EthereumClientError::TransactionReceiptNotFound(
                        e.to_string(),
                    ))
                }
            }
        }
    }

    fn contract(&self) -> Auditability::AuditabilityInstance<(), &DynProvider> {
        Auditability::new(self.contract, &self.provider)
    }
}

sol! {
    #[sol(rpc)]
    contract Auditability {
        function store(string index, bytes32 root) external;
        function proof(string index, bytes32 root) external view returns (bool);
    }
}
