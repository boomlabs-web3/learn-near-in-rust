use near_units::{parse_gas, parse_near};
use serde_json::json;
use workspaces::prelude::*;
use workspaces::{network::Sandbox, Account, AccountId, BlockHeight, Contract, Worker};

const WASM_FILEPATH: &str = "../../export/vending-machine.wasm";
const TOKEN_CONTRACT_ACCOUNT: &str = "contract_account_name_on_testnet.testnet";
const BLOCK_HEIGHT: BlockHeight = 12345;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(WASM_FILEPATH)?;

    // dev deploy of vending-machine contract
    let vendor_contract = worker.dev_deploy(&wasm).await?;
    
    // get archived testnet
    let testnet = workspaces::testnet_archival().await?;
    let token_contract_id: AccountId = TOKEN_CONTRACT_ACCOUNT.parse()?;

    // pull down ft contract from testnet
    let token_contract = worker
        .import_contract(&token_contract_id, &testnet)
        .initial_balance(parse_near!("1000 N"))
        .block_height(BLOCK_HEIGHT)
        .transact()
        .await?;

    // create root accounts(TLA) & sub-account
    let root = worker.root_account();
    
    let jay = root
        .create_subaccount(&worker, "jay")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    // initialize token contract
    jay
        .call(&worker, token_contract_id, "new_default_meta")
        .args_json(serde_json::json!({
            "owner_id": vendor_contract.id(),
        }))?
        .transact()
        .await?;

    // initialize vendor contract
    jay
        .call(&worker, vendor_contract.id(), "new")
        .args_json(serde_json::json!({
            "token_contract": token_contract_id,
        }))?
        .transact()
        .await?;

    // begin tests
    test_get_token(&jay, &token_contract, &vendor_contract, &worker).await?;
    Ok(())
}

async fn test_get_token(
    user: &Account,
    token_contract: &Contract,
    vendor_contract: &Contract,
    worker: &Worker<Sandbox>,
) -> anyhow::Result<()> {
    // staking storage fee for user in token contract
    user
        .call(&worker, token_contract.id(), "storage_deposit")
        .args_json(serde_json::json!({}))?
        .deposit(parse_near!("1.25 mN"))
        .transact()
        .await?;

    let message: String = user
        .call(&worker, vendor_contract.id(), "get_token")
        .args_json(json!({"amount": "2"}))?
        .deposit(parse_near!("2 N"))
        .max_gas()
        .transact()
        .await?
        .json()?;

    assert_eq!(message, format!("2 tokens for {} are minted", user.id()).to_string());
    println!("      Passed ✅ gets default message");
    Ok(())
}