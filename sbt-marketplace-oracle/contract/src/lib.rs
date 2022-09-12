/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen, PanicOnDefault, AccountId, env};
use near_sdk::collections::{Vector};

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    request_queue: Vector<(AccountId, String, AccountId)>,
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    pub fn validate(&self){

    }

    pub fn request_validation(&mut self, account_to_validate: (AccountId, String)){
        let (account_id, public_key) = account_to_validate;
        self.request_queue.push(&(account_id, public_key, env::predecessor_account_id()));
    }

    pub fn get_next_request(&self) -> Option<(AccountId, String)> {
        let queue_size = self.request_queue.len();
        if queue_size == 0 {
            return None;
        }

        let (account_id, public_key, _) = self.request_queue.get(queue_size - 1).unwrap();
        return Some((account_id, public_key));
    }
}

