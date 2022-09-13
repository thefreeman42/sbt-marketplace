use crate::*;

// TODO: update/remove offer

trait SBTMarketplaceOffers {
    fn add_offer(&mut self, listing_id: ListingId);

    fn accept_offer(&mut self, listing_id: ListingId, permission: SBTPermission);
}

#[near_bindgen]
impl SBTMarketplaceOffers for Contract {
    #[payable]
    fn add_offer(&mut self, listing_id: ListingId) {
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

    fn accept_offer(&mut self, listing_id: ListingId, permission: SBTPermission) {
        let id = listing_id;

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
            // 80% of tx to owner
            let owner_amount = price * 8 / 10;
            let providers: Vec<AccountId> = listing.tokens.iter().map(|l| l.sbt_contract_id.clone()).collect();
            // 15% of tx to the providers of each token, split evenly
            let provider_amount = price * 3 / 20 / (providers.len() as u128);
            for provider in providers.iter() {
                Promise::new(provider.clone()).transfer(provider_amount);
            }
            Promise::new(listing.account_id).transfer(owner_amount);
            // 5% of tx remains as marketplace fee
        }
    }
}