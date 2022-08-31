# Unit Test & Integration Test

In this section, we will learn unit test & integration test.
* Docs Reference: [Test](https://docs.near.org/develop/testing/introduction)  
* Github Reference: [`near/workspaces-rs`](https://github.com/near/workspaces-rs)      

![image](https://user-images.githubusercontent.com/96561121/187455447-fdfb75ff-dc32-4243-9a40-9a8c3236d6bb.png)
* **Unit tests** are great for ensuring that functionality works as expected at an insolated, functional-level. Unit tests are fast & light-weighted, but can't test things be happened on NEAR network, such as calculating gas fee or cross-contract calls.
* **Integration tests** provide the ability to have end-to-end testing that includes cross-contract calls, proper user accounts, access to state, structured execution outcomes, and more. Integration test can be executed on testnet or mocked local environment called `sandbox`.

## Unit Test
```bash
git checkout 6.contract/test
```
Checkout to 6th branch.   
In unit test, we will use `test` module of `cargo` in rust. You may refer the official [cargo book](https://doc.rust-lang.org/cargo/guide/tests.html).
```rust
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests
```
`tests` is module of test.   
`#[cfg(..)]` is attribute macro which tells compiler to compile `tests` module only when the conditions in parentheses are true.
```rust
fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder
        .current_account_id(accounts(0))
        .signer_account_id(predecessor_account_id.clone())
        .predecessor_account_id(predecessor_account_id);
    builder
}
```
`get_context` method is used to build testing context.

## Integration Test
Integration test is written in `near-meetup/integration-test/src/lib.rs`.
```rust
const WASM_FILEPATH: &str = "../export/vending-machine.wasm";
const TOKEN_CONTRACT_ACCOUNT: &str = "contract.boomlabs.testnet";
const BLOCK_HEIGHT: BlockHeight = 96257940;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(WASM_FILEPATH)?;
	let vendor_contract = worker.dev_deploy(&wasm).await?;
```
`main` method returns `anyhow::Result<()>`.   
`anyhow` is a crate that deals with error handling, making it more robust for developers.   
`worker` is our gateway towards interacting with our sandbox environment.   
`wasm` is wasm file compiled from vending-machine contract code.   
`vendor_contract` is the dev-deployed contract on sandbox the developer interacts with.
```rust
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
```
This code is written to [spoon](https://github.com/near/workspaces-rs#spooning---pulling-existing-state-and-contracts-from-mainnettestnet) FT contract deployed in testnet.   
`testnet` is worker to interact with testnet archival node.   
`token_contract_id` is `"contract.boomlabs.testnet"`.   
`token_contract` is pulled down from testnet.   
[`.import_contract`](https://docs.rs/workspaces/latest/workspaces/struct.Worker.html#method.import_contract) method receives arguments as `AccountId` and `Worker`, imports contract from testnet, and returns `ImportContractTransaction` object.   
`.initial_balance` method sets initial balance of token contract as 1000 $NEAR.   
[`.transact().await`](https://docs.rs/workspaces/latest/workspaces/operations/struct.Transaction.html#method.transact) method executes `ImportContractTransaction` and receives result.
```rust
// create root accounts(TLA) & sub-account
let root = worker.dev_create_account().await?;

let jay = root
    .create_subaccount(&worker, "jay")
    .initial_balance(parse_near!("30 N"))
    .transact()
    .await?
    .into_result()?;
```
`root` is TLA account used in local `sandbox`.   
`jay` is sub-account of `root`.   
[`.create_subaccount`](https://docs.rs/workspaces/latest/workspaces/struct.Account.html#method.create_subaccount) method receives arguments as `str` and `worker` and returns [`CreateAccountTransaction`](https://docs.rs/workspaces/latest/workspaces/operations/struct.CreateAccountTransaction.html) object. This object will be executed by `.transact().await` method.
```rust
// initialize token contract
jay
    .call(&worker, &token_contract_id, "new_default_meta")
    .args_json(serde_json::json!({
        "owner_id": vendor_contract.id(),
    }))?
    .transact()
    .await?;

// initialize vendor contract
vendor_contract
    .call(&worker, "new")
    .args_json(serde_json::json!({
        "token_contract": &token_contract_id,
    }))?
    .transact()
    .await?;
```
This calls contract.   
First line is equivalent with `near call 'token_contract' new_default_meta '{"owner_id": "vendor_contract"}' --acountId 'jay'` command in `near-cli`.
```rust
test_get_token(&jay, &token_contract, &vendor_contract, &worker).await?;

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
```
If `jay` calls `get_token` method in `vendor_contract`, and if received message is equal with `"2 tokens for jay are minted"`, then test is passed with `"      Passed ✅ gets default message"` message.
## Execution
```bash
npm run test
```
![image](https://user-images.githubusercontent.com/96561121/187462904-6b427f58-314a-4d05-a735-3778359cccc1.png)
Unit tests are passed.
![image](https://user-images.githubusercontent.com/96561121/187463014-d83b86ba-7574-4084-90aa-dc3d625168cd.png)
Integration test is passed after processing of 3 blocks in local `sandbox` environment.
