use near_sdk::collections::{Vector};
use near_sdk::{env, near_bindgen};
use std::hash::{Hasher};
use std::collections::hash_map::DefaultHasher;
use crate::*;

#[near_bindgen]
impl Contract {
    pub fn view_listings(&self) -> Vec<SBTListing> {
        return self.listings_by_id.values().collect();
    }

    pub fn add_listing(&mut self,
        tokens: Vec<SBTTokenLocator>,
        price: Option<U128>
    ) -> U64 {
        require!(tokens.len() > 0, "Listing must include at least 1 token");

        let account_id = env::predecessor_account_id();
        let id = Self::get_listing_id(&tokens, &account_id);
        require!(self.listings_by_id.get(&id).is_none(), "Listing for this token set already exists");

        // TODO check that the SBTs are owned by the signer account
        
        let listing = SBTListing {
            id: id.clone(),
            account_id: account_id.clone(),
            tokens: tokens.clone(),
            price: match price {
                Some(p) => Some(u128::from(p)),
                None => None
            }
        };

        self.listings_by_id.insert(&id.clone(), &listing);
        let mut accounts_listings = self
            .listings_for_account
            .get(&account_id)
            .unwrap_or(Vector::new(StorageKey::ListingsForAccount{account_id: account_id.clone()}));
        accounts_listings.push(&listing.id);
        self.listings_for_account.insert(&account_id, &accounts_listings);
        U64(id)
    }

    fn get_listing_id(tokens: &Vec<SBTTokenLocator>, account: &AccountId) -> ListingId {
        let mut hasher = DefaultHasher::new();
        tokens.hash(&mut hasher);
        account.hash(&mut hasher);
        hasher.finish()
    }

    pub fn view_own_offers(&self) -> Vec<SBTListingOffer> {
        return self.offers_for_account
            .get(&env::predecessor_account_id())
            .unwrap_or(UnorderedMap::new(StorageKey::OffersByAccount))
            .values().collect();
    }

    #[payable]
    pub fn add_offer(&mut self, listing_id: ListingId) {
        let found_listing = self.listings_by_id.get(&listing_id);
        require!(found_listing.is_some(), "Listing does not exist");
        let listing = found_listing.unwrap();

        let offering_account = &env::predecessor_account_id();
        require!(listing.account_id != *offering_account, "Cannot submit offer for own listing");
        if let Some(ref all_listings_by_offering_account) = self.offers_for_account.get(&offering_account) {
            if let Some(ref _previous_offer) = all_listings_by_offering_account.get(&listing_id) {
                panic!("There is an offer in place for this listing by this account");
            }
        }

        let offered_price = &env::attached_deposit();
        if let Some(ref expected_price) = &listing.price {
            if offered_price < &u128::from(*expected_price) {
                panic!("Deposit does not match listed price");
            }
        }

        let offer = SBTListingOffer {
            listing_id: listing_id.clone(),
            offering_account_id: offering_account.clone(),
            offered_price: {
                if *offered_price > 0 { Some(offered_price.clone()) } else { None }
            }
        };

        let mut account_offers = self
            .offers_for_account
            .get(&offering_account)
            .unwrap_or(UnorderedMap::new(StorageKey::OffersForAccount{account_id: offering_account.clone()}));
        account_offers.insert(&listing_id, &offer);
        self.offers_for_account.insert(&offering_account, &account_offers);
    }

    pub fn accept_offer(&mut self, listing_id: U64, permission: SBTPermission) {
        let id = u64::from(listing_id);

        let listing: SBTListing = {
            require!(self.listings_by_id.get(&id).is_some(), "Listing does not exist");
            let found = self.listings_by_id.get(&id).unwrap();
            require!(found.account_id == env::predecessor_account_id(), "Cannot accept offer for another account's listing");
            found
        };

        require!(permission.body.accounts.len() > 0, "At least 1 account must be given permission");
        // TODO: handle accepting multiple offers at the same time
        require!(permission.body.accounts.len() == 1, "WIP: Only 1 offer can be accepted");

        let offering_account = permission.body.accounts[0].clone();
        let offer : SBTListingOffer = {
            let acc_offers = self.offers_for_account.get(&offering_account);
            require!(acc_offers.is_some(), "Account does not have offers on this contract");
            let acc_list_offers = acc_offers.unwrap();
            require!(acc_list_offers.get(&id).is_some(), "Offer does not exist");
            acc_list_offers.get(&id).unwrap()
        };

        self.create_permission(permission);

        self.offers_for_account
            .get(&offer.offering_account_id)
            .unwrap()
            .remove(&id);
        if let Some(ref price) = offer.offered_price {
            let owner_amount = price * 8 / 10;
            let providers: Vec<AccountId> = listing.tokens.iter().map(|l| l.sbt_contract_id.clone()).collect();
            let provider_amount = price * 2 / 10 / (providers.len() as u128);
            for provider in providers.iter() {
                Promise::new(provider.clone()).transfer(provider_amount);
            }
            Promise::new(listing.account_id).transfer(owner_amount);
        }
    }
}