use near_sdk::{env, ext_contract, log, PromiseResult, AccountId};
use near_sdk::json_types::U128;

#[ext_contract(ext_token_contract)]
trait TokenContract {
    // External contract에 저장된 메소드의 인터페이스
    fn ft_mint(
      &mut self,
      receiver_id: AccountId,
      amount: U128,
    );

    fn ft_burn(
      &mut self,
      amount: U128,
    );
}

pub fn did_promise_succeed() -> bool {
  if env::promise_results_count() != 1 {
    log!("Expected a result on the callback");
    return false;
  }

  match env::promise_result(0) {
    PromiseResult::Successful(_) => true,
    _ => false,
  }
}