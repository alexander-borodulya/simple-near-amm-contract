use near_sdk::{env};

use crate::{Contract, external::{ext_token, ext_self}};

impl Contract {
    
    // TODO
    #[allow(dead_code)]
    pub (crate) fn internal_query_external_metadata(&mut self) {
        ext_token::ext(self.token_a.clone()) // External Contract Token instance
            .get_metadata() // External Metadata Promise
                .then(ext_self::ext(env::current_account_id()) // External Contract Self
                    .on_get_metadata(self.token_a.clone()));

        ext_token::ext(self.token_b.clone()) // External Contract Token instance
            .get_metadata() // External Metadata Promise
                .then(ext_self::ext(env::current_account_id()) // External Contract Self
                    .on_get_metadata(self.token_b.clone()));
    }    
}
