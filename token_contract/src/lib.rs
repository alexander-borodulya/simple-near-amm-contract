use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, Vector};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, StorageUsage, require, log};

pub mod ft_core;
pub mod events;
pub mod metadata;
pub mod storage;
pub mod internal;

use crate::metadata::*;
use crate::events::*;

/// The image URL for the default icon
const _DATA_IMAGE_TOKEN_ICON: &str = "data:image/webp;base64,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";

/// The specific version of the standard we're using
pub const FT_METADATA_SPEC: &str = "ft-0.0.1";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
// #[serde(crate = "near_sdk::serde")]
pub struct Contract {
    /// Total supply of all tokens.
    pub total_supply: Balance,

    /// Keep track of each account's balances
    pub accounts: LookupMap<AccountId, Balance>,

    /// Kepp track of each account's key (for debug purposes only)
    pub accounts_keys: Vector<AccountId>,

    /// The bytes for the largest possible account ID that can be registered on the contract 
    pub bytes_for_longest_account_id: StorageUsage,

    /// Metadata for the contract itself
    pub metadata: LazyOption<FungibleTokenMetadata>,

    /// AMM Wallet Account
    pub amm_account: Option<AccountId>,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    Accounts,
    Metadata
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract with arguments...
    #[init]
    pub fn new(
        owner_id: AccountId,
        name: String,
        symbol: String,
        total_supply: U128,
        decimals: u8,
    ) -> Self {
        require!(!env::state_exists(), format!("The contract {name} has initialized!"));
        let mut this = Self {
            total_supply: total_supply.into(),
            
            // Set the bytes for the longest account ID to 0 temporarily until it's calculated later
            bytes_for_longest_account_id: 0, 
            
            // Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            accounts: LookupMap::new(StorageKey::Accounts.try_to_vec().unwrap()),

            // Accounts ID's only
            accounts_keys: Vector::new(b"s".to_vec()),
            
            metadata: LazyOption::new(
                StorageKey::Metadata.try_to_vec().unwrap(),
                Some(&FungibleTokenMetadata {
                        spec: FT_METADATA_SPEC.to_string(),
                        name: name.clone(),
                        symbol: symbol.clone(),
                        total_supply: total_supply.into(),
                        icon: None, // icon: Some(DATA_IMAGE_TOKEN_ICON.to_string()),
                        reference: None,
                        reference_hash: None,
                        decimals,
                    },
                ),
            ),

            amm_account: None,
        };

        // Measure the bytes for the longest account ID and store it in the contract.
        this.measure_bytes_for_longest_account_id();

        // Register the owner's account and set their balance to the total supply.
        this.internal_register_account(&owner_id);
        this.internal_deposit(&owner_id, total_supply.into());
        
        // Emit an event showing that the FTs were minted
        FtMint {
            owner_id: &owner_id,
            amount: &total_supply,
            memo: Some(format!("Initial token supply of {}, {}({}) were minted", total_supply.0, name, symbol).as_str()),
        }
        .emit();

        // Return the Contract object
        this
    }

    /// Getter for cross-contract call
    pub fn get_metadata(&self) -> FungibleTokenMetadata {
        self.ft_metadata()
    }

    #[payable]
    pub fn create_wallet(&mut self, sender_id: AccountId, amount: Balance) {
        // require!(self.amm_id.is_none(), "amm has been registered");
        let receiver_id = env::predecessor_account_id();
        self.amm_account = Some(receiver_id.clone());
        self.internal_register_account(&receiver_id);
        self.transfer_from(sender_id, receiver_id, amount)
    }

    #[payable]
    pub fn transfer_from(&mut self, sender_id: AccountId, receiver_id: AccountId, amount: Balance) {
        require!(Some(env::predecessor_account_id()) == self.amm_account, "Only AMM contract can transfer from");
        require!(sender_id != receiver_id, "Sender and receiver should be different");
        require!(amount > 0, format!("The amount should be a positive number: {}", amount));

        self.internal_withdraw(&sender_id, amount);
        if !self.accounts.contains_key(&receiver_id) {
            self.internal_register_account(&receiver_id);
        }
        self.internal_deposit(&receiver_id, amount);
        FtTransfer {
            old_owner_id: &sender_id,
            new_owner_id: &receiver_id,
            amount: &U128(amount),
            memo: None,
        }
        .emit();
    }

    /// For the debug purposes only
    pub fn print_accounts(&self) {
        log !("Contract: {}, meta: {}", self.total_supply, self.metadata.get().unwrap().name);
        for id in self.accounts_keys.iter() {
            if let Some(balance) = self.accounts.get(&id) {
                log!("Account: {}, balance: {}", id, balance);
            }
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    use crate::ft_core::FungibleTokenCore;
    use crate::storage::StorageManagement;

    const TOTAL_SUPPLY: Balance = 1_000_000_000_000_000;

    use super::*;
    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        let name = "Token A";
        let symbol = "tkn_A";
        let total_supply = 10000000000000000000000;
        let decimals = 18;
        let metadata = FungibleTokenMetadata {
            spec: FT_METADATA_SPEC.to_string(),
            name: name.to_string(),
            symbol: symbol.to_string(),
            total_supply,
            icon: None,
            reference: None,
            reference_hash: None,
            decimals,
        };

        testing_env!(context.build());
        let contract = Contract::new(
            accounts(1).into(),
            name.into(),
            symbol.into(),
            near_sdk::json_types::U128(total_supply),
            decimals,
        );

        testing_env!(context.is_view(true).build());
        assert_eq!(contract.ft_total_supply().0, total_supply);
        assert_eq!(contract.ft_balance_of(accounts(1)).0, total_supply);
        assert_eq!(contract.get_metadata(), metadata);
    }

    #[test]
    fn test_transfer() {
        let mut context = get_context(accounts(2));
        testing_env!(context.build());

        // owner_id: AccountId,
        // name: String,
        // symbol: String,
        // total_supply: U128,
        // decimals: u8,

        let mut token_contract = Contract::new(
            accounts(2).into(),
            "Token A".into(),
            "tkn_A".into(),
            TOTAL_SUPPLY.into(),
            18,
        );

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(token_contract.storage_balance_bounds().min.into())
            .predecessor_account_id(accounts(1))
            .build());

        token_contract.storage_deposit(None, None);
        
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(2))
            .build());

        let transfer_amount = TOTAL_SUPPLY / 3;
        token_contract.ft_transfer(
            accounts(1), 
            transfer_amount.into(),
            None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        
            assert_eq!(
            token_contract.ft_balance_of(accounts(2)).0,
            (TOTAL_SUPPLY - transfer_amount)
        );
        assert_eq!(token_contract.ft_balance_of(accounts(1)).0, transfer_amount);
    }
    
}
