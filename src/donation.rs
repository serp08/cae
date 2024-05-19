use near_sdk::{env, near, AccountId, Promise, NearToken};

use crate::TransactionContract;

#[near(serializers = [json])]
pub struct Donation {
    pub donor: AccountId,
    pub donee: String,
    pub sum: NearToken,
}

//#[near]
impl TransactionContract {
    pub fn transfer(&mut self, amt: NearToken) {
        let donor: AccountId = env::predecessor_account_id();
        let prev_donations:NearToken = *self.donations.get(&donor).unwrap_or(&NearToken::from_near(0));
        Promise::new(self.donee.clone()).transfer(amt);
        self.donations.insert(donor, prev_donations.saturating_add(amt));
    }

    pub fn change_donee(&mut self, don: AccountId) {
        self.donee = don;
    }

    pub fn get_donee(&self) -> &AccountId {
        &self.donee
    }

    pub fn total(&self) -> NearToken {
        let donor: AccountId = env::predecessor_account_id();
        return *self.donations.get(&donor).unwrap_or(&NearToken::from_near(0));
    }
}