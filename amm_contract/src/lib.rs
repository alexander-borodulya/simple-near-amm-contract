use metadata::FungibleTokenMetadata;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, AccountId, env, PanicOnDefault, assert_self, log, ext_contract, Balance, require};

////////////////////////////////
/// External

#[ext_contract(ext_token)]
trait ExtToken {
    fn create_wallet(&mut self, sender_id: AccountId, amount: Balance);
    fn get_metadata(&self) -> FungibleTokenMetadata;
    fn ft_transfer(
        &mut self, 
        receiver_id: AccountId, 
        amount: U128, 
        memo: Option<String>);
    fn transfer_from(&mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        amount: Balance);
}

#[ext_contract(ext_self)]
trait ExtSelf {
    fn on_get_metadata(
        &mut self,
        contract_id: AccountId, 
        #[callback] metadata: FungibleTokenMetadata);
    fn on_ft_deposit(
        &mut self,
        from_balance: Balance,
        to_balance: Balance,
        contract_id: AccountId,
        receiver_id: AccountId,
        amount: Balance,
    );
    fn on_update_balances(
        &mut self,
        sender_post_balance: Balance, 
        receiver_post_balance: Balance
    );
}

/// External
////////////////////////////////

pub mod external;
pub mod internal;
pub mod metadata;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
pub struct TokenContractInfo {
    ticker: String,
    decimals: u8,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
pub struct AmmContractInfo {
    token_a: TokenContractInfo,
    token_b: TokenContractInfo,
    ratio: u128
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    owner_id: AccountId, 
    token_a: AccountId,
    token_a_meta: FungibleTokenMetadata,
    token_b: AccountId,
    token_b_meta: FungibleTokenMetadata,
    tokens_ratio: u128,
    pub tokens: LookupMap<AccountId, FungibleTokenMetadata>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner_id: AccountId, 
        token_a_contract_id: AccountId, 
        token_b_contract_id: AccountId
    ) -> Self {
        assert!(!env::state_exists(), "Contract already initialized: {}", owner_id);

        let this = Self {
            owner_id: owner_id.clone(),
            token_a: token_a_contract_id.clone(),
            token_a_meta: FungibleTokenMetadata::default(),
            token_b: token_b_contract_id.clone(),
            token_b_meta: FungibleTokenMetadata::default(),
            tokens_ratio: 0,
            tokens: LookupMap::new(b"t".to_vec()),
        };

        ext_token::ext(token_a_contract_id.clone()) // External Contract Token instance
            .get_metadata() // External Metadata Promise
                .then(ext_self::ext(env::current_account_id()) // External Contract Self
                    .on_get_metadata(token_a_contract_id.clone()));

        ext_token::ext(token_b_contract_id.clone()) // External Contract Token instance
            .get_metadata() // External Metadata Promise
                .then(ext_self::ext(env::current_account_id()) // External Contract Self
                    .on_get_metadata(token_b_contract_id.clone()));

        // Creates wallet for the AMM in both Token А and Token B contracts
        ext_token::ext(this.token_a.clone()) // External Contract Token instance
            .create_wallet(
                owner_id.clone(), 
                this.token_a_meta.total_supply
            );
        ext_token::ext(this.token_b.clone()) // External Contract Token instance
            .create_wallet(
                owner_id.clone(), 
                this.token_b_meta.total_supply
            );

        this
    }

    #[private]
    pub fn on_get_metadata(
        &mut self, 
        contract_id: AccountId, 
        #[callback] metadata: FungibleTokenMetadata)
    {
        assert_self();
        log!("on_get_metadata: contract_id: {} metadata {:?}", contract_id, metadata);
        
        if self.token_a == contract_id {
            self.token_a_meta = metadata.clone();
        } else if self.token_b == contract_id {
            self.token_b_meta = metadata.clone();
        } else {
            env::panic_str(format!("Unsupported contract id: {}", contract_id).as_str());
        }

        self.tokens.insert(&contract_id, &metadata);

        self.update_tokens_ratio();
    }

    // Update tokens ratio based on metadata of tokens A and B
    pub fn update_tokens_ratio(&mut self) -> u128 {
        let minted_units_token_a = self.token_a_meta.total_supply / 10_u128.pow(self.token_a_meta.decimals.into());
        let minted_units_token_b = self.token_b_meta.total_supply / 10_u128.pow(self.token_b_meta.decimals.into());
        self.tokens_ratio = minted_units_token_a * minted_units_token_b;
        self.get_tokens_ratio()
    }

    // Returns tokens ratio
    pub fn get_tokens_ratio(&self) -> u128 {
        self.tokens_ratio
    }

    // Returns all the metadata: Token A + Token B + tokens ratio
    pub fn tokens_full_info(&self) -> AmmContractInfo {
        AmmContractInfo {
            token_a: TokenContractInfo {
                ticker: self.token_a_meta.symbol.clone(),
                decimals: self.token_a_meta.decimals
            },
            token_b: TokenContractInfo {
                ticker: self.token_b_meta.symbol.clone(),
                decimals: self.token_b_meta.decimals
            },
            ratio: self.get_tokens_ratio()
        }
    }

    // Reterns a metadata by the token id
    pub fn token_info_by_id(&self, token_contract_id: AccountId) -> TokenContractInfo {
        if self.token_a == token_contract_id {
            TokenContractInfo {
                ticker: self.token_a_meta.symbol.clone(),
                decimals: self.token_a_meta.decimals
            }
        } else if self.token_b == token_contract_id {
            TokenContractInfo {
                ticker: self.token_b_meta.symbol.clone(),
                decimals: self.token_b_meta.decimals
            }
        } else {
            env::panic_str(format!("Unsupported token contract id: {}", token_contract_id).as_str());
        }
    }

    // Send tokens A, in return, receives token B...
    pub fn deposit_contract(&self, amount: Balance) {
        // The function that returns opposite token contract id
        let get_return_contract_id = |contract_id: AccountId| -> AccountId {
            if self.token_a == contract_id {
                self.token_b.clone()
            } else if self.token_b == contract_id {
                self.token_a.clone()
            } else {
                env::panic_str(format!("Unsupported token contract id: {}", contract_id).as_str());
            }
        };

        let sender_id = env::predecessor_account_id();

        // Evaluate opposite token contract
        let contract_id_for_the_return = get_return_contract_id(sender_id.clone());

        // Get metadata for token contracts
        let sender_token_meta = self.tokens
            .get(&sender_id)
            .unwrap_or_else(|| env::panic_str(format!("deposit_contract: unknown token contract: {}", &sender_id).as_str()));
        let return_token_meta = self.tokens
            .get(&sender_id)
            .unwrap_or_else(|| env::panic_str(format!("deposit_contract: unknown token contract: {}", &sender_id).as_str()));

        // Total decimal units
        let sender_decimal = 10_u128.pow(sender_token_meta.decimals as u32);
        let receiver_decimal = 10_u128.pow(return_token_meta.decimals as u32);

        // Total tokens amount
        let sender_tokens_amount = amount * sender_decimal; // A or B

        let sender_post_amount = sender_tokens_amount + sender_token_meta.total_supply;

        let receiver_post_amount = self.tokens_ratio / (sender_post_amount / sender_decimal) * receiver_decimal;

        let receiver_tokens_amount = return_token_meta.total_supply - receiver_post_amount; // B or A
        
        // ext_token::ext(contract_id_for_the_return.clone())
        ext_token::ext(self.token_a.clone())
            .transfer_from(sender_id.clone(), env::current_account_id(), sender_tokens_amount)
            .then(
                ext_self::ext(env::current_account_id())
                    .on_ft_deposit(
                        sender_post_amount,
                        receiver_post_amount,
                        contract_id_for_the_return,
                        sender_id,
                        receiver_tokens_amount,
                    ),
            );
    }

    pub fn on_fn_transfer(&self) {
        log!("AMM: on_fn_transfer: TODO implementation...");
    }

    pub fn on_ft_deposit(
        &mut self,
        from_balance: Balance,
        to_balance: Balance,
        contract_id: AccountId,
        receiver_id: AccountId,
        amount: Balance,
    ) {
        require!(env::predecessor_account_id() == env::current_account_id(), "Calling from another context is not allowed");
        ext_token::ext(contract_id)
            .transfer_from(env::current_account_id(), receiver_id, amount)
            .then(
                ext_self::ext(env::current_account_id())
                    .on_update_balances(from_balance, to_balance),
            );
    }

    // pub fn callback_update_tickers(&mut self, a_ticker_after: Balance, b_ticker_after: Balance) {
    pub fn on_update_balances(
        &mut self,
        sender_post_balance: Balance, 
        receiver_post_balance: Balance
    ) {
        require!(env::predecessor_account_id() == env::current_account_id(), "Calling from another context is not allowed");
        self.token_a_meta.total_supply = sender_post_balance;
        self.token_b_meta.total_supply = receiver_post_balance;
        self.update_tokens_ratio();
    }

    /// The owner of the contract can transfer a certain amount of tokens A or B to the contract account, thereby changing the ratio K.
    #[payable]
    pub fn deposit_token_contract(
        &mut self, 
        token_contract_id: AccountId,
        amount: Balance)
    {
        require!(env::predecessor_account_id() == self.owner_id, "Calling from another context is not allowed");

        let d = if token_contract_id == self.token_a {
            10_u128.pow(self.token_a_meta.decimals as u32)
        } else if token_contract_id == self.token_b {
            10_u128.pow(self.token_b_meta.decimals as u32)
        } else {
            env::panic_str(format!("Unsupported token contract: {}", token_contract_id).as_str());
        };

        let token_supply = if token_contract_id == self.token_a {
            self.token_a_meta.total_supply
        } else if token_contract_id == self.token_b {
            self.token_b_meta.total_supply
        } else {
            env::panic_str(format!("Unsupported token contract: {}", token_contract_id).as_str());
        };

        let final_amout_for_transfer = amount * d;

        let post_amount = amount * token_supply;

        let opposite_post_amount = if token_contract_id == self.token_a {
            self.token_b_meta.total_supply
        } else if token_contract_id == self.token_b {
            self.token_a_meta.total_supply
        } else {
            env::panic_str(format!("Unsupported token contract: {}", token_contract_id).as_str());
        };
        
        ext_token::ext(token_contract_id.clone())
            .transfer_from(
                self.owner_id.clone(), 
                env::current_account_id(), 
                final_amout_for_transfer)
            .then(
                ext_self::ext(env::current_account_id())
                    .on_update_balances(
                        post_amount, 
                        opposite_post_amount),
            );
    }
}
