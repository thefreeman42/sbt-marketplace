use crate::*;

// TODO: update/remove offer

trait SBTMarketplaceOffers {
    fn add_offer(&mut self, listing_id: ListingId);

    fn view_offers(&self, account_id: AccountId) -> Vec<SBTListingOffer>;

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
        if let Some(ref all_listings_by_offering_account) = self.offers_by_account.get(&offering_account) {
            if all_listings_by_offering_account.contains(&listing_id) {
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
                if *offered_price > 0 { Some(U128(offered_price.clone())) } else { None }
            }
        };

        self.offers_by_id.insert(&(listing_id.clone(), offering_account.clone()), &offer);
        let mut account_offers = self
            .offers_by_account
            .get(&offering_account)
            .unwrap_or(UnorderedSet::new(StorageKey::OffersByAccountListing{account_id: offering_account.clone()}));
        account_offers.insert(&listing_id);
        self.offers_by_account.insert(&offering_account, &account_offers);
        let mut offers_for_account = self
            .offers_for_account
            .get(&listing.account_id)
            .unwrap_or(UnorderedSet::new(StorageKey::OffersForAccountOffers{account_id: listing.account_id.clone()}));
        offers_for_account.insert(&(listing_id.clone(), offering_account.clone()));
        self.offers_for_account.insert(&listing.account_id, &offers_for_account);
    }

    fn view_offers(&self, account_id: AccountId) -> Vec<SBTListingOffer> {
        let offer_keys = self.offers_for_account
            .get(&account_id)
            .unwrap_or(UnorderedSet::new(StorageKey::OffersForAccount))
            .to_vec();
        offer_keys.iter().map(|key| self.offers_by_id.get(&key).unwrap()).collect()
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
            let acc_offers = self.offers_by_account.get(&offering_account);
            require!(acc_offers.is_some(), "Account does not have offers on this contract");
            let acc_list_offers = acc_offers.unwrap();
            require!(acc_list_offers.contains(&id), "Offer does not exist");
            self.offers_by_id.get(&(id.clone(), offering_account.clone())).unwrap()
        };

        self.create_permission(permission);

        self.offers_by_id.remove(&(id.clone(), offering_account.clone()));
        // TODO remove offers correctly from everywhere

        if let Some(ref price_json) = offer.offered_price {
            let price = u128::from(*price_json);
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