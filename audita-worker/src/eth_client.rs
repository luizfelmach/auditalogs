use alloy::{
    hex,
    network::EthereumWallet,
    primitives::{Address, B256},
    providers::{DynProvider, ProviderBuilder},
    signers::local::PrivateKeySigner,
    sol,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EthClientError {
    #[error("Invalid private key: {0}")]
    InvalidPrivateKey(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Invalid contract address: {0}")]
    InvalidContractAddress(String),

    #[error("Failed to decode hex: {0}")]
    HexDecodeError(String),

    #[error("Transaction send failed: {0}")]
    TransactionSendError(String),

    #[error("Failed to fetch transaction receipt: {0}")]
    ReceiptFetchError(String),
}

pub struct EthClient {
    provider: DynProvider,
    contract: Address,
}

impl EthClient {
    pub async fn new(url: String, contract: String, pk: String) -> Result<Self, EthClientError> {
        let signer: PrivateKeySigner = pk
            .parse()
            .map_err(|_| EthClientError::InvalidPrivateKey(pk.clone()))?;
        let wallet = EthereumWallet::from(signer.clone());
        let url = url
            .parse()
            .map_err(|_| EthClientError::InvalidUrl(url.clone()))?;
        let provider = ProviderBuilder::new().wallet(wallet).on_http(url);
        let provider = DynProvider::new(provider);
        let contract = contract
            .parse()
            .map_err(|_| EthClientError::InvalidContractAddress(contract.clone()))?;

        Ok(Self { provider, contract })
    }

    pub async fn store(&self, index: &String, hash: &String) -> Result<String, EthClientError> {
        let contract = Auditability::new(self.contract, &self.provider);
        let hash_bytes =
            hex::decode(hash).map_err(|_| EthClientError::HexDecodeError(hash.clone()))?;
        let hash_b256 = B256::from_slice(&hash_bytes);

        let tx = contract
            .store(index.clone(), hash_b256)
            .send()
            .await
            .map_err(|e| EthClientError::TransactionSendError(e.to_string()))?;

        let receipt = tx
            .get_receipt()
            .await
            .map_err(|e| EthClientError::ReceiptFetchError(e.to_string()))?;

        Ok(format!("{}", receipt.transaction_hash))
    }
}

sol! {
    #[sol(rpc)]
    contract Auditability {
        function store(string index, bytes32 root) external;
        function proof(string index, bytes32 root) external view returns (bool);
    }
}
