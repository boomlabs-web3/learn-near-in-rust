use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen, PanicOnDefault,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Counter {
    value: u64,
}

#[near_bindgen]
impl Counter {
    #[init]
    pub fn new(value: u64) -> Self {
        log!("Custom counter initialization!");
        Self { value }
    }
    
    #[payable]
    pub fn increment(&mut self) {
        self.value += 1;
    }
    
    #[private]
    pub fn get_count(&self) -> u64 {
        self.value
    }
}
