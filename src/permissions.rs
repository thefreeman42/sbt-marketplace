use crate::*;

#[near_bindgen]
impl Contract {

    pub fn sbt_permissions_metadata(&self) -> SBTPermissionsContractMetadata {
        self.contract_metadata.clone()
    }

    fn sbt_permissions_impl(&self, token: SBTTokenLocator) -> Vec<Signature> {
        let mut result: Vec<String> = Vec::new();

        if !self
            .permissions_for_token
            .contains_key(&token.contract_key())
        {
            return Vec::new();
        }

        let permissions_for_contract = self
            .permissions_for_token
            .get(&token.contract_key())
            .unwrap();

        let star: TokenId = "*".to_string();

        if !permissions_for_contract.contains_key(&token.token_id.clone())
            && !permissions_for_contract.contains_key(&star)
        {
            return Vec::new();
        }

        if permissions_for_contract.contains_key(&token.token_id.clone()) {
            for token_permission in permissions_for_contract
                .get(&token.token_id.clone())
                .unwrap()
                .iter()
            {
                result.push(token_permission.clone());
            }
        }
        if permissions_for_contract.contains_key(&star) {
            for star_permission in permissions_for_contract.get(&star.clone()).unwrap().iter() {
                result.push(star_permission.clone());
            }
        }

        result
    }

    pub fn sbt_permissions(
        &self,
        token: SBTTokenLocator,
        from_index: Option<u64>,
        limit: Option<u64>,
    ) -> Vec<SBTPermission> {
        let result = self.sbt_permissions_impl(token);

        let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
        require!(limit != 0, "Cannot provide limit of 0.");
        let start_index: u128 = from_index.map(From::from).unwrap_or_default();

        if start_index > result.len() as u128 {
            return Vec::new();
        }

        result
            .iter()
            .skip(start_index as usize)
            .take(limit)
            .map(|signature| self.permissions_by_signature.get(&signature).unwrap())
            .collect()
    }

    pub fn sbt_permissions_count(&self, token: SBTTokenLocator) -> u64 {
        self.sbt_permissions_impl(token).len() as u64
    }

    // pub fn sbt_permissions_for_account(&self, account_id: AccountId, from_index: u64, limit: Option<u64>) -> Vec<SBT> {}
    // pub fn sbt_permissions_count_for_account(&self, account_id: AccountId) -> u64 {}

    fn verify_permission(&self, _permission: &SBTPermission) {
        // TODO: permission.body to json
        // Validate signature with json
    }

    pub fn create_permission(&mut self, permission: SBTPermission) {
        self.verify_permission(&permission);

        if self
            .permissions_by_signature
            .contains_key(&permission.signature.clone())
        {
            panic!("Permission with signature already exists");
        }

        self.permissions_by_signature
            .insert(&permission.signature.clone(), &permission);

        for token in permission.body.sbt_tokens {
            let mut contract_permissions = self
                .permissions_for_token
                .get(&token.contract_key())
                .unwrap_or(LookupMap::new(StorageKey::PermissionsForTokenContract {
                    chain_id: token.chain_id.clone(),
                    sbt_contract_id: token.sbt_contract_id.clone(),
                }));

            let mut token_permissions =
                contract_permissions
                    .get(&token.token_id)
                    .unwrap_or(Vector::new(StorageKey::PermissionsForTokenContractToken {
                        chain_id: token.chain_id.clone(),
                        sbt_contract_id: token.sbt_contract_id.clone(),
                        token_id: token.token_id.clone(),
                    }));

            token_permissions.push(&permission.signature.clone());
            contract_permissions.insert(&token.token_id, &token_permissions);
            self.permissions_for_token
                .insert(&token.contract_key(), &contract_permissions);
        }
    }
}
