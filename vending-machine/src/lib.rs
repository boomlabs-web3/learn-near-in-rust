use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, require, near_bindgen, AccountId, Balance, Gas, Promise, PromiseResult, PanicOnDefault};
use near_sdk::json_types::U128;

const ONE_YOCTO: Balance = 1;
const ONE_NEAR: Balance = 1_000_000_000_000_000_000_000_000;
const DECIMAL: Balance = 100_000_000;

const GAS_FEE: Gas = Gas(10_000_000_000_000); // 10TGAS

use crate::external_traits::*;

mod external_traits;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
//defining the contract struct that holds state
pub struct VendingMachine {
    pub token_contract: AccountId
}

#[near_bindgen]
impl VendingMachine {
    #[init]
    #[private] // Public - but only callable by env::current_account_id()
    pub fn new(token_contract: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
        token_contract,
        }
    }
    
    pub fn get_token (
        &self, 
        amount: U128,
    ) -> Promise {
        let mut amount: Balance = amount.into();
        require!( amount * ONE_NEAR <= env::attached_deposit() );
        amount = amount * DECIMAL;
        let amount: U128 = amount.into();
        let promise = ext_token_contract::ext(self.token_contract.clone())
            .with_attached_deposit(ONE_YOCTO)
            .with_static_gas(GAS_FEE)
            .ft_mint(env::signer_account_id(), amount);
        
        return promise.then(
            Self::ext(env::current_account_id())
                .with_static_gas(GAS_FEE)
                .callback()
        )
    }

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
}