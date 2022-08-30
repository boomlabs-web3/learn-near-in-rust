# Cross Contract Calls
In this section, we will learn cross-contract calls by deploying vending-machine contract which interacts with pre-deployed FT contract.
* Docs Reference: [Cross-Contract Calls](https://docs.near.org/develop/contracts/crosscontract)
* Github Reference: [`near-examples/docs-examples`](https://github.com/near-examples/docs-examples/tree/main/cross-contract-hello-rs)

![image](https://user-images.githubusercontent.com/96561121/187244014-19e7a22c-0315-4454-8601-af2041333b4f.png)
This is a diagram explains what we build.
1) User calls `get_token` method of `vendor.sender.testnet` contract.
2) `vendor.sender.testnet` contract calls `ft_mint` method of `token.sender.testnet` with `sender.testnet` & `amount` as arguments.
3) `token.sender.testnet` mints token for `sender.testnet`. as result of `ft_mint` method, FT contract returns `promise` containing `"{ amount } tokens for { sender.testnet } are minted”` message.
4) `vendor.sender.testnet` contract executes `callback` method to process received `promise`.
5) User gets error message or `"{ amount } tokens for { sender.testnet } are minted”` message.

## Let's move to code!
```bash
git checkout 5.contract/cross-contract-call
```
Checkout to 5th branch.   

```
near-meetup
├─ token
│  ├─ src
│  │   └─ lib.rs
│  └─ target
│
├─ vending-machine
│  ├─ src
│  │  ├─ external_traits.rs
│  │  └─ lib.rs
│  └─ target
│
└─ export
   ├─ token.wasm
   └─ vending-machine.wasm
```
This is hierarchy of file system. Code of FT contract is stored in `near-meetup/token/src/`, and code of vending-machine is stored in `near-meetup/vending-machine/src/`. And compiled wasm files are stored in `near-meetup/export/`.
## External Traits
```rust
// near-meetup/vending-machine/src/external_traits.rs

#[ext_contract(ext_token_contract)]
trait TokenContract {
    // External contract에 저장된 메소드의 인터페이스
    fn ft_mint(
      &mut self,
      receiver_id: AccountId,
      amount: U128,
    ) -> String;

    fn ft_burn(
      &mut self,
      amount: U128,
    );
}
```
This file defines interface of counterpart contract to be called.   
Defines counterpart contract's trait by using `#[ext_contract(..)]` attribute macro.
## Cross Contract Calls & Promises
```rust
pub fn get_token (
    &self, 
    amount: U128,
) -> Promise {
	..
    let promise = ext_token_contract::ext(self.token_contract.clone())
        .with_attached_deposit(ONE_YOCTO)
        .with_static_gas(GAS_FEE)
        .ft_mint(env::signer_account_id(), amount);
	..
}
```
This is part of `get_token` method. This method returns `Promise` object.   
Cross-contract calls work in network by creating two kinds of `Promise`,
1) `Promise` to execute external contract's method (`Promise.create`)
2) Call-back `Promise` to execute internal method as result of external cross contract calls (`Promise.then`)   

Both `Promise` take the same arguments:
```rust
external_trait::ext("external_address")
    .with_attached_deposit(DEPOSIT)
    .with_static_gas(GAS)
    .method(arguments);
```
* The address of the contract you want to interact with
* The method that you want to execute
* The (encoded) arguments to pass to the method
* The amount of GAS to use (deducted from the attached Gas)
* The amount of NEAR to attach (deducted from your contract’s balance)

## Callback
```rust
pub fn get_token (
    &self, 
    amount: U128,
) -> Promise {
	..
    return promise.then(
        Self::ext(env::current_account_id())
            .with_static_gas(GAS_FEE)
            .callback()
		)
}
```
Callback promise should be executed after execution of previous promise. `promise.then(..)` syntax, called 'promise chaining', is used to deal with.   
In this example, internal `callback` method is called by result of promise chaining, using `Self::ext(env::current_account_id())`.
```rust
#[private]
pub fn callback(&self) -> String {
    if !did_promise_succeed() {
        log!("Error on calling token contract");
        return "".to_string();
    } else {
        let result: String = match env::promise_result(0) {
            PromiseResult::Successful(value) => near_sdk::serde_json::from_slice::<String>(&value).unwrap(),
            _ => { log!("Error on calling token contract"); return "".to_string(); },
        };

        result
    }
}
```
As a result of `callback` method,`"{ amount } tokens for { sender.testnet } are minted”` string contained in `PromiseResult::Successful(_)` should be returned.
## Execution
```bash
near create-account "vendor".$USER --masterAccount $USER --initialBalance 10 && near create-account "token".$USER --masterAccount $USER --initialBalance 10
```
Create sub-accounts to deploy vending-machine contract & token contract each.
```bash
yarn build

near deploy --accountId "vendor".$USER --wasmFile export/vending-machine.wasm && near deploy --accountId "token".$USER --wasmFile export/token.wasm
```
Compile vending-machine contract & token contract to wasm file and deploy each to sub-accounts created above.
```bash
near call "token".$USER new '{"owner_id": "vendor.'$USER'", "metadata": { "spec": "ft-1.0.0", "name": "BOOM LABS TOKEN", "symbol": "BOOM", "icon": "'$ICON'", "decimals": 8 }}' --accountId $USER

near call "vendor".$USER new '{"token_contract": "token.'$USER'"}' --accountId "vendor".$USER
```
Initialize each contract.
```bash
near call "token".$USER storage_deposit '' --accountId $USER --amount 0.00125

near call "vendor".$USER get_token '{"amount": "3"}' --amount 3 --accountId $USER --gas 100000000000000
```
Register my account to storage of token contract and call `get_token` method of vending-machine contract.

![image](https://user-images.githubusercontent.com/96561121/187454197-33989b2e-e4c6-4da7-ae03-15eb0096cf3f.png)
`‘3 tokens for sender.testnet are minted’`
## NEXT STEP: [Unit Test & Integration Test](https://github.com/boomlabs-web3/near-meetup/tree/6.contract/test)
