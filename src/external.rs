// use near_sdk::{ext_contract};
// use crate::*;

// pub const TGAS: u64 = 1_000_000_000_000;
// pub const NO_DEPOSIT: u128 = 0;
// pub const XCC_SUCCESS: u64 = 1;

// #[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
// #[serde(crate = "near_sdk::serde")]
// pub struct OwnerConsent {
//     pub signed_ids: String,
//     pub public_key: PublicKey,
//     pub signature: String,
// }

// #[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
// #[serde(crate = "near_sdk::serde")]
// pub enum AccessKeyNeeded {
//     None,
//     FunctionCall,
//     FullAccess,
// }

// #[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
// #[serde(crate = "near_sdk::serde")]
// struct SBTMetadata {
//     pub issued_at: Option<String>,
//     pub updated_at: Option<String>,
//     pub extra: Option<String>,
// }

// #[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
// #[serde(crate = "near_sdk::serde")]
// pub struct SBT {
//     pub id: TokenId,
//     pub owner_id: AccountId,
//     pub parent_id: Option<TokenId>,
//     pub owner_consent: Option<OwnerConsent>,
//     pub reference: Option<String>,
//     pub key_needed_for_access: AccessKeyNeeded,
//     pub metadata: Option<SBTMetadata>,
// }

// #[ext_contract(sbt_marketplace)]
// pub trait Callbacks {
//   fn listing_confirmation_callback(
//     &self,
//     listing: SBTListing, 
//     tokens_to_confirm: Vec<SBTTokenLocator>
//   );
// }

// #[ext_contract(sbt)]
// pub trait SBTContract {
//   fn sbt_token(&self, token_id: TokenId) -> SBT;
// }