use near_sdk::{ext_contract, AccountId};

use crate::metadata::*;

#[ext_contract(ext_token)]
pub trait ExtToken {
    fn get_metadata(&self) -> FungibleTokenMetadata;
    // fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128);
}

#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn on_get_metadata(
        &mut self,
        contract_id: AccountId, 
        #[callback] metadata: FungibleTokenMetadata);
}
