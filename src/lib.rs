// Reference to: https://docs.near.org
use near_sdk::store::UnorderedMap;
use near_sdk::{near, AccountId, NearToken, PanicOnDefault};

mod donation;

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct TransactionContract {
    pub donee: AccountId,
    pub donations: UnorderedMap<AccountId, NearToken>,
}

#[near]
impl TransactionContract {
    #[init]
    #[private] // only callable by the contract's account
    pub fn init(donee: AccountId) -> Self {
        Self {
            donee,
            donations: UnorderedMap::new(b"d"),
        }
    }

    pub fn access_donee(&self) -> &AccountId {
        &self.donee
    }

    #[private] // only callable by the contract's account
    pub fn alter_donee(&mut self, new_donee: AccountId) {
        self.donee = new_donee;
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;
    use near_sdk::NearToken;

    const DONEE: &str = "donee";
    const ONE_NEAR: NearToken = NearToken::from_near(1);

    #[test]
    fn initializes() {
        let contract = TransactionContract::init(DONEE.parse().unwrap());
        assert_eq!(
            contract.donee,
            DONEE.parse::<AccountId>().unwrap().to_string()
        );
    }

    #[test]
    fn donate() {
        let mut contract = TransactionContract::init(DONEE.parse().unwrap());

        // Make a donation
        set_context("donor_a", ONE_NEAR);
        contract.donate();
        let first_donation = contract.get_donation_for_account("donor_a".parse().unwrap());

        // Check the donation was recorded correctly
        assert_eq!(
            u128::from(first_donation.total_amount),
            ONE_NEAR.as_yoctonear()
        );

        // Make another donation
        set_context("donor_b", ONE_NEAR.saturating_mul(2));
        contract.donate();
        let second_donation = contract.get_donation_for_account("donor_b".parse().unwrap());

        // Check the donation was recorded correctly
        assert_eq!(
            u128::from(second_donation.total_amount),
            ONE_NEAR.saturating_mul(2).as_yoctonear()
        );

        // User A makes another donation on top of their original
        set_context("donor_a", ONE_NEAR);
        contract.donate();
        let first_donation = contract.get_donation_for_account("donor_a".parse().unwrap());

        // Check the donation was recorded correctly
        assert_eq!(
            u128::from(first_donation.total_amount),
            ONE_NEAR.saturating_mul(2).as_yoctonear()
        );

        assert_eq!(u64::from(contract.number_of_donors()), 2);
    }

    // Auxiliar fn: create a mock context
    fn set_context(predecessor: &str, amount: NearToken) {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor.parse().unwrap());
        builder.attached_deposit(amount);

        testing_env!(builder.build());
    }
}
