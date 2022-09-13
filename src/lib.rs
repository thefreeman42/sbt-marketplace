use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector, UnorderedMap};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::{U64, U128};
use near_sdk::{near_bindgen, require, Promise, AccountId, PublicKey, BorshStorageKey};
use std::hash::{Hash};

pub use crate::external::*;
pub use crate::permissions::*;
pub use crate::marketplace::*;

mod external;
mod permissions;
mod marketplace;

pub type TokenId = String;
pub type Signature = String;
pub type ListingId = u64;

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
    pub signature: Signature,      // The signature generated when signing a JSONified representation of body
    pub public_key: PublicKey,     // The key used to sign the body
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SBTPermissionsContractMetadata {
    pub spec: String,              // SBT specification (sbt-permissions-1.0.0)
    pub name: Option<String>,      // The name of the SBT permissions provider
    pub symbol: Option<String>,    // The symbol of the SBT permissions provider
    pub base_uri: Option<String>,  // Base uri for the SBT permissions server
    pub reference: Option<String>, // Any additional info for the SBT permissions provider
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SBTListing {
    pub id: ListingId,
    pub account_id: AccountId,
    pub tokens: Vec<SBTTokenLocator>,
    pub price: Option<u128>
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SBTListingOffer {
    pub listing_id: ListingId,
    pub offering_account_id: AccountId,
    pub offered_price: Option<u128>
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
    OffersByAccount,
    OffersForAccount {
        account_id: AccountId
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    contract_metadata: SBTPermissionsContractMetadata,
    owner_id: AccountId,
    permissions_by_signature: LookupMap<Signature, SBTPermission>,
    permissions_for_token: LookupMap<(String, AccountId), LookupMap<TokenId, Vector<Signature>>>,
    listings_by_id: UnorderedMap<ListingId, SBTListing>,
    listings_for_account: LookupMap<AccountId, Vector<ListingId>>,
    offers_for_account: LookupMap<AccountId, UnorderedMap<ListingId, SBTListingOffer>>
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            owner_id: "default".parse().unwrap(),
            contract_metadata: SBTPermissionsContractMetadata {
                spec: "sbt-permissions-0.0.1".to_string(),
                name: None,
                symbol: None,
                base_uri: None,
                reference: None,
            },
            permissions_by_signature: LookupMap::new(StorageKey::PermissionsBySignature),
            permissions_for_token: LookupMap::new(StorageKey::PermissionsForToken),
            listings_by_id: UnorderedMap::new(StorageKey::ListingsById),
            listings_for_account: LookupMap::new(StorageKey::ListingsByAccount),
            offers_for_account: LookupMap::new(StorageKey::OffersByAccount),
        }
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, metadata: SBTPermissionsContractMetadata) -> Self {
        // TODO: verify spec
        Self {
            owner_id: owner_id,
            contract_metadata: metadata,
            permissions_by_signature: LookupMap::new(StorageKey::PermissionsBySignature),
            permissions_for_token: LookupMap::new(StorageKey::PermissionsForToken),
            listings_by_id: UnorderedMap::new(StorageKey::ListingsById),
            listings_for_account: LookupMap::new(StorageKey::ListingsByAccount),
            offers_for_account: LookupMap::new(StorageKey::OffersByAccount),
        }
    }
}