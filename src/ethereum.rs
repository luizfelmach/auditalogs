use alloy::primitives::Address;
use alloy::providers::ProviderBuilder;
use alloy::providers::RootProvider;
use alloy::sol;
use alloy::transports::http::Client;
use std::error::Error;

sol! {
  #[sol(rpc)]
  "artifacts/Auditability.sol",
}

type ContractType = Auditability::AuditabilityInstance<
    alloy::transports::http::Http<Client>,
    RootProvider<alloy::transports::http::Http<Client>>,
>;

pub fn Contract(url: &str, contract_addr: Address) -> Result<ContractType, Box<dyn Error>> {
    let rpc_url = url.parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);
    Ok(Auditability::new(contract_addr, provider))
}
