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
    #[init]
    pub fn new(
        owner_id: AccountId,
        metadata: FungibleTokenMetadata,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        let mut this = Self {
            token: FungibleToken::new(b"a".to_vec()),
            metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
            controller: owner_id.clone(), 
        };
        this.token.internal_register_account(&owner_id);
        this
    }

    #[payable]
    pub fn ft_mint(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
    ) -> String {
        // Full-Access Key로 sign된 function call인지 확인.
        // https://welcome.near.university/developers/contract-patterns/assert_one_yocto-forced-confirmation
        assert_one_yocto();
        
        // caller mint authority 있는지 확인하기
        let caller_id = env::predecessor_account_id();
        require!(caller_id == self.controller, "Only controller can call mint");
        
        let mut amount: Balance = amount.into();
        require!(amount > 0, "The amount should be a positive number");

        self.token.internal_deposit(&receiver_id, amount);
        //  event emit 
        near_contract_standards::fungible_token::events::FtMint {
            owner_id: &receiver_id,
            amount: &amount.into(),
            memo: Some((format!("Token for {} is minted", &receiver_id)).as_str()),
        }.emit();
        let decimal: u32 = self.metadata.get().unwrap().decimals.into();
        let unit: u128 = 10;
        amount = amount / (unit.pow(decimal));
        format!("{} tokens for {} are minted", &amount.to_string(), &receiver_id).to_string()
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


#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    // accounts: "alice", "bob", "charlie", "danny", "eugene", "fargo"
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::MockedBlockchain;
    use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
    use near_sdk::{testing_env, Balance};

    use super::*;

    const AMOUNT: Balance = 2_000_000_000;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        // current_account_id = "alice", signer_account_id = "bob"
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new(accounts(1).into(), FungibleTokenMetadata {
            spec: "ft-1.0.0".to_string(),
            name: "BOOM LABS TEST Token".to_string(),
            symbol: "TEST".to_string(),
            icon: Some("".to_string()),
            reference: None,
            reference_hash: None,
            decimals: 8,
        });
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.controller, AccountId::new_unchecked("bob".to_string()));
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }

    #[test]
    fn test_mint() {
        let mut context = get_context(accounts(2));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1).into(), FungibleTokenMetadata {
            spec: "ft-1.0.0".to_string(),
            name: "BOOM LABS TEST Token".to_string(),
            symbol: "TEST".to_string(),
            icon: Some("".to_string()),
            reference: None,
            reference_hash: None,
            decimals: 8,
        });
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(contract.storage_balance_bounds().min.into())
            .predecessor_account_id(accounts(1))
            .build());
        // Paying for account registration, aka storage deposit
        contract.storage_deposit(None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(1))
            .build());
        let mint_amount = AMOUNT;
        let string = contract.ft_mint(accounts(1), mint_amount.into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert_eq!(contract.ft_balance_of(accounts(1)).0, AMOUNT);
        assert_eq!(string, "20 tokens for bob are minted".to_string());
    }

    #[test]
    #[should_panic(expected = "Only controller can call mint")]
    fn test_mint_authority() {
        let mut context = get_context(accounts(2));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1).into(), FungibleTokenMetadata {
            spec: "ft-1.0.0".to_string(),
            name: "BOOM LABS TEST Token".to_string(),
            symbol: "TEST".to_string(),
            icon: Some("".to_string()),
            reference: None,
            reference_hash: None,
            decimals: 8,
        });
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(contract.storage_balance_bounds().min.into())
            .predecessor_account_id(accounts(1))
            .build());
        // Paying for account registration, aka storage deposit
        contract.storage_deposit(None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(2))
            .build());
        let mint_amount = AMOUNT;
        contract.ft_mint(accounts(1), mint_amount.into());
    }

    #[test]
    #[should_panic(expected = "Requires attached deposit of exactly 1 yoctoNEAR")]
    fn test_mint_assert_one_yocto() {
        let mut context = get_context(accounts(2));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1).into(), FungibleTokenMetadata {
            spec: "ft-1.0.0".to_string(),
            name: "BOOM LABS TEST Token".to_string(),
            symbol: "TEST".to_string(),
            icon: Some("".to_string()),
            reference: None,
            reference_hash: None,
            decimals: 8,
        });
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(contract.storage_balance_bounds().min.into())
            .predecessor_account_id(accounts(1))
            .build());
        // Paying for account registration, aka storage deposit
        contract.storage_deposit(None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(0)
            .predecessor_account_id(accounts(2))
            .build());
        let mint_amount = AMOUNT;
        contract.ft_mint(accounts(1), mint_amount.into());
    }
}