use alloy::{
    primitives::Address,
    providers::{ProviderBuilder, RootProvider},
    sol,
    transports::http::Client,
};
use std::error::Error;
use std::str::FromStr;

sol! {
  #[sol(rpc)]
  "artifacts/Auditability.sol",
}

type ContractType = Auditability::AuditabilityInstance<
    alloy::transports::http::Http<Client>,
    RootProvider<alloy::transports::http::Http<Client>>,
>;

pub fn contract(url: &str, contract_addr: &str) -> Result<ContractType, Box<dyn Error>> {
    let rpc_url = url.parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);
    Ok(Auditability::new(
        Address::from_str(contract_addr)?,
        provider,
    ))
}
