use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::Map;
// use near_sdk::json_types::{U128};
use near_sdk::{
    env, near_bindgen, AccountId,
    // Balance,
    // ext_contract, EpochHeight, Promise, PublicKey,
};
// use serde::{Deserialize, Serialize};
// use uint::construct_uint;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct FileHashContract {
    /// Persistent map from file hash to account ID
    pub files: Map<Vec<u8>, AccountId>,
}

impl Default for FileHashContract {
    fn default() -> Self {
        let this = Self {
            files: Map::new(b"u".to_vec()), // init map
        };
        this
    }
}

impl FileHashContract {

    // public functions

    pub fn add_file(&mut self, hash: &Vec<u8>) {
        self.files.insert(hash, &env::current_account_id());
    }

    pub fn get_account(&self, hash: &Vec<u8>) -> AccountId {
        self.files.get(hash).unwrap_or_default()
    }

    pub fn remove_file(&mut self, hash: &Vec<u8>) {
        if self.get_account(hash) == env::current_account_id() {
            self.files.remove(hash);
        }
    }
}

/********************************
Tests
********************************/
#[cfg(test)]
mod test_utils;
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env};
    // mark individual unit tests with #[test] for them to be registered and fired
    #[test]
    fn check_owner() {
        // VMContextBuilder is defined in `test_utils.rs`
        // part of writing unit tests is setting up a mock context
        testing_env!(VMContextBuilder::new()
            .current_account_id(alice())
            .finish());
        let account = env::current_account_id();

        let file:Vec<u8> = vec![1, 2, 3];
        let mut contract = FileHashContract::default();
        contract.add_file(&file);
        let returned_account = contract.get_account(&file);
        
        assert_eq!(account, returned_account);
    }

    #[test]
    fn check_remove() {
        // VMContextBuilder is defined in `test_utils.rs`
        // part of writing unit tests is setting up a mock context
        testing_env!(VMContextBuilder::new()
            .current_account_id(alice())
            .finish());

        let file:Vec<u8> = vec![1, 2, 3];
        let mut contract = FileHashContract::default();
        contract.add_file(&file);
        contract.remove_file(&file);
        let returned_account = contract.get_account(&file);
        println!("check_remove: file account == {}", returned_account);

        assert_eq!(returned_account, "");
    }

    #[test]
    fn cannot_remove() {
        // VMContextBuilder is defined in `test_utils.rs`
        // part of writing unit tests is setting up a mock context
        testing_env!(VMContextBuilder::new()
            .current_account_id(alice())
            .finish());
        let account = env::current_account_id(); // alice

        let file:Vec<u8> = vec![1, 2, 3];
        let mut contract = FileHashContract::default();
        contract.add_file(&file);
        // switch env context
        testing_env!(VMContextBuilder::new()
            .current_account_id(bob()) // switched current_account_id [msg.sender] to bob
            .finish());

        contract.remove_file(&file);
        let returned_account = contract.get_account(&file);
        println!("cannot_remove: file account == {}", returned_account);

        assert_eq!(returned_account, account);
    }

}