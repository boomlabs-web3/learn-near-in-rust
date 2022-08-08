# Deploy Simple Fungible Token Contract on NEAR blockchain
In this section, we will deploy simple FT contract on NEAR blockchain to learn smart contract deployment and initialization of smart contract.
* Docs Reference: [Fungible Tokens](https://docs.near.org/develop/relevant-contracts/ft)
* Github Reference: [`near-examples/FT`](https://github.com/near-examples/FT)

```bash
# Modify this to your account address
export USER="sender.testnet"

# Compile contract code to wasm file & deploy to NEAR blockchain
yarn build && near deploy --accountId $USER --wasmFile export/main.wasm

# Initialize deployed FT contract
near call $USER new '{"owner_id": "'$USER'", "total_supply": "1000", "metadata": { "spec": "ft-1.0.0", "name": "BOOM LABS TOKEN", "symbol": "BOOM", "decimals": 8 }}' --accountId $USER
```
Modify first line, `"sender.testnet"` to your account address.

## Deep dive to contract, what's going on in the contract?
```rust
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
}
```
This is FT contract struct, which has simple field - token & metadata.
```rust
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
        this
    }
}

// near-contract-standards-4.0.0/src/fungible_token/core_impl.rs
impl FungibleToken {
    pub fn new<S>(prefix: S) -> Self
    where
        S: IntoStorageKey,
    {
        let mut this =
            Self { accounts: LookupMap::new(prefix), total_supply: 0, account_storage_usage: 0 };
        this.measure_account_storage_usage();
        this
    }
}

// near-sdk-4.0.0/src/collections/lazy_option.rs
impl<T> LazyOption<T>
where
    T: BorshSerialize + BorshDeserialize,
{
    pub fn new<S>(storage_key: S, value: Option<&T>) -> Self
    where
        S: IntoStorageKey,
    {
        let mut this = Self { storage_key: storage_key.into_storage_key(), el: PhantomData };
        if let Some(value) = value {
            this.set(value);
        }
        this
    }
    
    pub fn set(&mut self, value: &T) -> bool {
        self.set_raw(&Self::serialize_value(value))
    }
}
```
When you call `new` method with `"owner_id"`, `"total_supply"`, and `"metadata"` arguments,
`new` method initialize FT contract.
`new` method initialize FungibleToken object and Metadata object with storage key, "a" to byte vector and "m" to byte vectory corresponding. 

## NEXT STEP: [DAY 2, Upgrade Pre-Deployed Contract and Learn about Schema Migration](https://github.com/boomlabs-web3/near-meetup/tree/4.contract/upgraded-ft)
