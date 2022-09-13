/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen, PanicOnDefault, AccountId, env, Gas, ext_contract};
use near_sdk::collections::{Vector};

pub const TGAS: u64 = 1_000_000_000_000;

#[ext_contract(oracle)]
trait Oracle {
    fn request_validation(&self,
        to_validate_account: AccountId, to_validate_public_key: String, callback_message: Option<String>);
}
// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    oracle_account_id: AccountId
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(oracle_account_id: AccountId) -> Self{
        Self {
            oracle_account_id: oracle_account_id
        }
    }

    pub fn request(&mut self, account_id: AccountId, public_key: String, message: Option<String>) {
        oracle::ext(self.oracle_account_id.clone())
            .with_static_gas(Gas(200*TGAS))
            .request_validation(account_id.clone(), public_key, message);
    }

    pub fn on_sbt_marketplace_oracle_result(&self,
        account_id: AccountId, public_key: String, outcome: bool, memo: Option<String>) {
        assert!(env::predecessor_account_id() == self.oracle_account_id, "Only the oracle is allowed to call this method");
        log!("Oracle result: {} {} {} {}", account_id, public_key, outcome, memo.unwrap_or("".to_string()));
    }
}

