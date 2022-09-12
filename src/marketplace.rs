use near_sdk::collections::{Vector};
use near_sdk::{env, near_bindgen};
use crate::*;

#[near_bindgen]
impl Contract {
    pub fn view_listings(&self) -> Vec<SBTListing> {
        return self.listings_by_id.values().collect();
    }

    pub fn list_tokens(&mut self, listing: SBTListing) {
        if listing.tokens.len() < 1 {
            panic!("Listing must include at least 1 token");
        }
        // TODO id increment
        // TODO ownership check
        self.listings_by_id.insert(&listing.id.clone(), &listing);
        let account_id = env::predecessor_account_id();
        let mut accounts_listings = self
            .listings_for_account
            .get(&account_id)
            .unwrap_or(Vector::new(StorageKey::ListingsForAccount{account_id: account_id.clone()}));
        accounts_listings.push(&listing.id);
        self.listings_for_account.insert(&account_id, &accounts_listings);   
    }
}

// #[near_bindgen]
// impl Contract {
//     pub fn list_tokens(&self, listing: SBTListing) -> Promise {
//         if listing.tokens.len() < 1 {
//             panic!("At least one token must be included in the listing");
//         }
//         if self.listings_by_id.contains_key(&listing.id.clone()) {
//             panic!("Listing with id already exists");
//         }
        
//         let promise = self.confirm_next_token(listing, listing.tokens.clone());
//         return promise.then(
//             self.apply_listing(listing)
//         )
//     }

//     #[private]
//     pub fn confirm_next_token(&self, listing: SBTListing, tokens_to_confirm: Vec<SBTTokenLocator>) -> Promise {
//         let token = tokens_to_confirm.pop().unwrap();

//         let promise = sbt::ext(token.sbt_contract_id.clone())
//             //.with_static_gas(Gas(5*TGAS))
//             .sbt_token(token.token_id.clone());

//         return promise.then(
//             sbt_marketplace::ext(env::current_account_id())
//                 //.with_static_gas(Gas(5*TGAS))
//                 .listing_confirmation_callback(listing, tokens_to_confirm));
//     }

//     #[private]
//     pub fn apply_listing(&mut self, listing: SBTListing) {
//         self.listings_by_id.insert(&listing.id.clone(), &listing);
//         let account_id = env::predecessor_account_id();
//         let mut accounts_listings = self
//             .listings_for_account
//             .get(&account_id)
//             .unwrap_or(Vector::new(StorageKey::ListingsForAccount{account_id: account_id.clone()}));
//         accounts_listings.push(&listing.id);
//         self.listings_for_account.insert(&account_id, &accounts_listings);
//     }

//     #[payable]
//     pub fn add_offer(&mut self, _listing_id: ListingId) {

//     }
// }

// #[near_bindgen]
// impl Callbacks for Contract {
//     #[private]
//     fn listing_confirmation_callback(
//         &self,
//         listing: SBTListing,
//         tokens_to_confirm: Vec<SBTTokenLocator>
//     ) {
//         let sbt: SBT = promise_result_as_success().and_then(|value| {
//             Some(near_sdk::serde_json::from_slice::<SBT>(&value).unwrap())
//         });
//         if sbt.owner_id != listing.account_id {
//             panic!("All listed tokens must belong to the account creating the listing");
//         }
//         return;
//     }
// }