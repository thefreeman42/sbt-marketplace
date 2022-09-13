use crate::*;

trait SBTMarketplaceListings {
    fn view_listings(&self) -> Vec<SBTListing>;

    fn add_listing(&mut self,
        tokens: Vec<SBTTokenLocator>,
        price: Option<U128>,
    ) -> ListingId;
}

#[near_bindgen]
impl SBTMarketplaceListings for Contract {
    fn view_listings(&self) -> Vec<SBTListing> {
        return self.listings_by_id.values().collect();
    }

    fn add_listing(&mut self,
        tokens: Vec<SBTTokenLocator>,
        price: Option<U128>
    ) -> ListingId {
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
        id
    }
}

#[near_bindgen]
impl Contract {
    fn get_listing_id(tokens: &Vec<SBTTokenLocator>, account: &AccountId) -> ListingId {
        let mut hasher = DefaultHasher::new();
        tokens.hash(&mut hasher);
        account.hash(&mut hasher);
        hasher.finish().to_string()
    }
}