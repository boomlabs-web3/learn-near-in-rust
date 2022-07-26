use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;
use near_sdk::{assert_one_yocto, env, log, require, near_bindgen, AccountId, Balance, PanicOnDefault, PromiseOrValue};
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
    controller:AccountId,
}

#[near_bindgen]
impl Contract {
    // near-cli command: `near deploy --wasmFile out/main.wasm --initFunction "migrate" --initArgs "{}" --accountId $USER`
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

    // near-cli command: `near deploy --wasmFile out/main.wasm --initFunction "migrate_metadata" --initArgs '{"metadata": { "spec": "ft-1.0.0", "name": "BOOM LABS UPGRADED TOKEN", "symbol": "BOOM", "icon": "'$ICON'", "decimals": 4 }}' --accountId $USER`
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
            controller: env::predecessor_account_id(),
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

    #[payable]
    pub fn ft_mint(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
    ) {
        // Full-Access Key로 sign된 function call인지 확인.
        // https://welcome.near.university/developers/contract-patterns/assert_one_yocto-forced-confirmation
        assert_one_yocto();
        
        // caller mint authority 있는지 확인하기
        let caller_id = env::predecessor_account_id();
        require!(caller_id == self.controller, "Only controller can call mint");
        
        let amount: Balance = amount.into();
        require!(amount > 0, "The amount should be a positive number");

        self.token.internal_deposit(&receiver_id, amount);
        //  event emit 
        near_contract_standards::fungible_token::events::FtMint {
            owner_id: &receiver_id,
            amount: &amount.into(),
            memo: Some((format!("Token for {} is minted", &receiver_id)).as_str()),
        }.emit();
    }

    #[payable]
    pub fn ft_burn(
        &mut self,
        amount: U128,
    ) {
        assert_one_yocto();
        // caller_id == contract 호출한 사람.
        let caller_id = env::predecessor_account_id();
    
        let amount: Balance = amount.into();
        require!(amount > 0, "The amount should be a positive number");

        self.token.internal_withdraw(&caller_id, amount);
        //  event emit 
        near_contract_standards::fungible_token::events::FtBurn {
            owner_id: &caller_id,
            amount: &amount.into(),
            memo: Some((format!("{}'s Token is burned", &caller_id)).as_str()),
        }.emit();
    }

    fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
        log!("Closed @{} with {}", account_id, balance);
    }

    fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
        log!("Account @{} burned {}", account_id, amount);
    }

}

near_contract_standards::impl_fungible_token_core!(Contract, token, on_tokens_burned);
near_contract_standards::impl_fungible_token_storage!(Contract, token, on_account_closed);

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}