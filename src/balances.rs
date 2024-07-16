use std::collections::BTreeMap;

use num::{CheckedAdd, CheckedSub, Zero};

/// This is the Balances Module.
/// It is a simple module which keeps track of how much balance each account has in this state
/// machine.
#[derive(Debug)]
pub struct Pallet<AccountId, Balance> {
    // A simple storage mapping from accounts (`String`) to their balances (`u128`).
    balances: BTreeMap<AccountId, Balance>,
}

impl<AccountId, Balance> Pallet<AccountId, Balance>
where
    AccountId: Ord + Clone,
    Balance: Zero + CheckedSub + CheckedAdd + Copy,
{
    /// Create a new instance of the balances module.
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    /// Set the balance of an account `who` to some `amount`.
    pub fn set_balance(&mut self, who: &AccountId, amount: Balance) {
        self.balances.insert(who.clone(), amount);
    }

    /// Get the balance of an account `who`.
    /// If the account has no stored balance, we return zero.
    pub fn balance(&self, who: &AccountId) -> Balance {
        *self.balances.get(who).unwrap_or(&Balance::zero())
    }

    /// Transfer `amount` from one account to another.
    /// This function verifies that `from` has at least `amount` balance to transfer,
    /// and that no mathematical overflows occur.
    pub fn transfer(
        &mut self,
        caller: AccountId,
        to: AccountId,
        amount: Balance,
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("insufficient balance")?;

        let new_to_balance = to_balance
            .checked_add(&amount)
            .ok_or("max founds limit reached")?;

        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Pallet;

    #[test]
    fn init_balances() {
        let mut balances = Pallet::<String, u128>::new();

        assert_eq!(balances.balance(&"alice".to_string()), 0);

        balances.set_balance(&"alice".to_string(), 100);

        assert_eq!(balances.balance(&"alice".to_string()), 100);
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut balances = Pallet::<String, u128>::new();

        assert_eq!(
            balances.transfer("alice".to_string(), "bob".to_string(), 10),
            Err("insufficient balance")
        );

        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(
            balances.transfer("alice".to_string(), "bob".to_string(), 10),
            Ok(())
        );

        assert_eq!(balances.balance(&"alice".to_string()), 90);
        assert_eq!(balances.balance(&"bob".to_string()), 10);
    }
}
