use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector, UnorderedMap, UnorderedSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::{U128};
use near_sdk::{env, near_bindgen, require, Promise, AccountId, PublicKey, BorshStorageKey};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

//pub use crate::external::*;
pub use crate::permissions::*;
pub use crate::listings::*;
pub use crate::offers::*;

//mod external;
mod permissions;
mod listings;
mod offers;

pub type TokenId = String;
pub type Signature = String;
pub type ListingId = String;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Hash)]
#[serde(crate = "near_sdk::serde")]
pub struct SBTTokenLocator {
    pub chain_id: String,
    pub sbt_contract_id: AccountId,
    pub token_id: TokenId,
}

impl SBTTokenLocator {
    pub fn contract_key(&self) -> (String, AccountId) {
        return (self.chain_id.clone(), self.sbt_contract_id.clone());
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PermissionBody {
    pub sbt_tokens: Vec<SBTTokenLocator>,
    pub accounts: Vec<AccountId>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SBTPermission {
    pub body: PermissionBody,
    pub signature: Signature,
    pub public_key: PublicKey,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SBTPermissionsContractMetadata {
    pub spec: String,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub base_uri: Option<String>,
    pub reference: Option<String>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SBTListing {
    pub id: ListingId,
    pub account_id: AccountId,
    pub tokens: Vec<SBTTokenLocator>,
    pub price: Option<U128>
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SBTListingOffer {
    pub listing_id: ListingId,
    pub offering_account_id: AccountId,
    pub offered_price: Option<U128>
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    PermissionsBySignature,
    PermissionsForToken,
    PermissionsForTokenContract {
        chain_id: String,
        sbt_contract_id: AccountId,
    },
    PermissionsForTokenContractToken {
        chain_id: String,
        sbt_contract_id: AccountId,
        token_id: TokenId,
    },
    ListingsById,
    ListingsByAccount,
    ListingsForAccount {
        account_id: AccountId
    },
    OffersById,
    OffersByAccount,
    OffersByAccountListing {
        account_id: AccountId
    },
    OffersForAccount,
    OffersForAccountOffers {
        account_id: AccountId
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    contract_metadata: SBTPermissionsContractMetadata,
    //oracle_account_id: AccountId,
    owner_id: AccountId,
    permissions_by_signature: LookupMap<Signature, SBTPermission>,
    permissions_for_token: LookupMap<(String, AccountId), LookupMap<TokenId, Vector<Signature>>>,
    listings_by_id: UnorderedMap<ListingId, SBTListing>,
    listings_for_account: LookupMap<AccountId, Vector<ListingId>>,
    offers_by_id: UnorderedMap<(ListingId, AccountId), SBTListingOffer>,
    offers_by_account: LookupMap<AccountId, UnorderedSet<ListingId>>,
    offers_for_account: LookupMap<AccountId, UnorderedSet<(ListingId, AccountId)>>
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            owner_id: "default".parse().unwrap(),
            //oracle_account_id: "oracle_contract".parse().unwrap(),
            contract_metadata: SBTPermissionsContractMetadata {
                spec: "sbt-permissions-0.0.1".to_string(),
                name: Some("sbt-marketplace-nearcon".parse().unwrap()),
                symbol: Some("NCMP".parse().unwrap()),
                base_uri: None,
                reference: None,
            },
            permissions_by_signature: LookupMap::new(StorageKey::PermissionsBySignature),
            permissions_for_token: LookupMap::new(StorageKey::PermissionsForToken),
            listings_by_id: UnorderedMap::new(StorageKey::ListingsById),
            listings_for_account: LookupMap::new(StorageKey::ListingsByAccount),
            offers_by_id: UnorderedMap::new(StorageKey::OffersById),
            offers_by_account: LookupMap::new(StorageKey::OffersByAccount),
            offers_for_account: LookupMap::new(StorageKey::OffersForAccount)
        }
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner_id: AccountId,
        //oracle_account_id: AccountId,
        metadata: SBTPermissionsContractMetadata
    ) -> Self {
        Self {
            owner_id: owner_id,
            //oracle_account_id: oracle_account_id,
            contract_metadata: metadata,
            permissions_by_signature: LookupMap::new(StorageKey::PermissionsBySignature),
            permissions_for_token: LookupMap::new(StorageKey::PermissionsForToken),
            listings_by_id: UnorderedMap::new(StorageKey::ListingsById),
            listings_for_account: LookupMap::new(StorageKey::ListingsByAccount),
            offers_by_id: UnorderedMap::new(StorageKey::OffersById),
            offers_by_account: LookupMap::new(StorageKey::OffersByAccount),
            offers_for_account: LookupMap::new(StorageKey::OffersForAccount)
        }
    }
}