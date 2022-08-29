# Upgrade Pre-deployed Smart Contract
In this section, we will upgrade FT contract which was deployed in previous section.

* Docs Reference: [Schema Migration](https://welcome.near.university/developers/contract-patterns/schema-migration)

```bash
git checkout 4.contract/upgraded-ft
export USER="sender.testnet"
yarn build && near deploy --accountId $USER --wasmFile export/main.wasm
```
Modify second line, `"sender.testnet"` to your account address.

![image](https://user-images.githubusercontent.com/96561121/187223648-cce22b15-0adb-4962-a6ab-2b4e9d503b3f.png)
Enter y to proceed.

```bash
near view $USER ft_balance_of '{"account_id": "'$USER'"}'
near view $USER ft_metadata
```
To interact with newly deployed contract, type command above.

![image](https://user-images.githubusercontent.com/96561121/187236492-a9292b6e-fee5-4e47-b88e-fbd994a6e7c1.png)
Then `'Cannot deserialize the contract state.'` error occured.

## What happens with the contract?
```rust
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner_id: AccountId,
        total_supply: U128,
        metadata: FungibleTokenMetadata,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        let mut this = Self {
            token: FungibleToken::new(b"a".to_vec()),
            metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
        };
        this.token.internal_register_account(&owner_id);
        this.token.internal_deposit(&owner_id, total_supply.into());
        near_contract_standards::fungible_token::events::FtMint {
            owner_id: &owner_id,
            amount: &total_supply,
            memo: Some("Initial tokens supply is minted"),
        }
        .emit();
        this
    }
}
```
This is part of FT contract deployed in previous branch. `Contract` Struct only has `token` & `metadata` field.
```rust
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
    controller:AccountId,
}
```
This is Struct of newly deployed contract. `controller` field is added to `Contract` struct.
NEAR blockchain stores state in storage, independent with contract. And contract load & deserialize state from storage, execute code, serialize & store state to storage.
So as we re-deploy the contract, `Contract` struct is changed, but the state stored in storage is previous version, only have `token` & `metadata` field. Therefore error occured in deserialization of state. In order to resolve this error, we need to proceed **[Schema migration](https://welcome.near.university/developers/contract-patterns/schema-migration)**.
## Schema Migration
```rust
#[private]
#[init(ignore_state)]
pub fn migrate() -> Self {
    #[derive(BorshDeserialize)]
    pub struct OldContract {
        token: FungibleToken,
        metadata: LazyOption<FungibleTokenMetadata>,
    }

    let old: OldContract = env::state_read().unwrap();

    Self {
        token: old.token,
        metadata: old.metadata.into(),
        controller:env::predecessor_account_id(),
    }
}
```
Schema migration is quite simple.
First, declare `old` objects as `OldContract` struct which is same as previously deployed contract struct.
Then, load state from storage via `env::state_read().unwrap()`, and unwrap it to `old` object.
Finally, initialize `Contract` struct include `controller` field, and store it to storage.
```bash
near call $USER migrate '' --accountId $USER
```
You can call `migrate` method by typing command above.

![image](https://user-images.githubusercontent.com/96561121/187241258-50bddf75-82a6-4c5d-a79f-b115aa181f2f.png)
## Metadata Update
Also you can change metadata stored in storage.
```rust
#[private]
#[init(ignore_state)]
pub fn migrate_metadata(
    metadata: FungibleTokenMetadata,
) -> Self {
    #[derive(BorshDeserialize)]
    pub struct OldContract {
        token: FungibleToken,
        metadata: LazyOption<FungibleTokenMetadata>,
        controller:AccountId,
    }

    let old: OldContract = env::state_read().unwrap();
    metadata.assert_valid();
    Self {
        token: old.token,
        metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
        controller:old.controller,
    }
}
```
This is code of `migrate_metadata` method.
```bash
near deploy --wasmFile export/main.wasm --initFunction "migrate_metadata" --initArgs '{"metadata": { "spec": "ft-1.0.0", "name": "BOOM LABS UPGRADED TOKEN", "symbol": "BOOM", "icon": "'$ICON'", "decimals": 4 }}' --accountId $USER
```
You can change metadata of token by typing command above.

![image](https://user-images.githubusercontent.com/96561121/187242202-6baf2b5c-b6a1-4cd2-b4aa-f5d9e783319e.png)
Token changed. (Upper one -> bottom one)

## NEXT STEP: [Cross Contract Call](https://github.com/boomlabs-web3/near-meetup/tree/5.contract/cross-contract-call)
