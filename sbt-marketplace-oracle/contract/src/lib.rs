/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    log, near_bindgen, PanicOnDefault, AccountId, env, Gas, ext_contract,
};
use near_sdk::collections::{Vector};

pub const TGAS: u64 = 1_000_000_000_000;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct VerificationRequest {
    account_id: AccountId,
    public_key: String,
    callback_account_id: AccountId,
    callback_message: Option<String>
}

#[ext_contract(oracle_callback)]
trait Callbacks {
  fn on_sbt_marketplace_oracle_result(&self,
    account_id: AccountId, public_key: String, outcome: bool, memo: Option<String>);
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    request_queue: Vector<VerificationRequest>,
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(owner_id: AccountId) -> Self{
        Self {
            owner_id: owner_id,
            request_queue: Vector::new(b"requests".to_vec())
        }
    }

    pub fn request_validation(&mut self,
        to_validate_account: AccountId, to_validate_public_key: String, callback_message: Option<String>) {
        self.request_queue.push(&VerificationRequest {
            account_id: to_validate_account,
            public_key: to_validate_public_key,
            callback_account_id: env::predecessor_account_id(),
            callback_message: callback_message
        });
    }

    pub fn get_next_request(&self) -> Option<(AccountId, String)> {
        let queue_size = self.request_queue.len();
        if queue_size == 0 {
            return None;
        }

        let request = self.request_queue.get(queue_size - 1).unwrap();
        return Some((request.account_id, request.public_key));
    }

    pub fn apply_next_request(&mut self, action: (AccountId, String), result: bool) {
        assert!(env::signer_account_id() == self.owner_id, "Only the oracle can call this");
        let queue_size = self.request_queue.len();
        assert!(queue_size > 0, "No items in the queue");
        let request = self.request_queue.get(queue_size - 1).unwrap();
        let (applied_account_id, applied_public_key) = action;
        assert!(
            request.account_id == applied_account_id && request.public_key == applied_public_key,
            "Incorrect action");
        self.request_queue.pop();

        oracle_callback::ext(request.callback_account_id.clone())
            .with_static_gas(Gas(200*TGAS))
            .on_sbt_marketplace_oracle_result(
                request.account_id, request.public_key, result, request.callback_message);
    }
}

