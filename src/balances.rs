use std::collections::BTreeMap;

use num::{CheckedAdd, CheckedSub, Zero};

use crate::support::DispatchResult;

pub trait Config: crate::system::Config {
    type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

/// This is the Balances Module.
/// It is a simple module which keeps track of how much balance each account has in this state
/// machine.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    // A simple storage mapping from accounts (`String`) to their balances (`u128`).
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the balances module.
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    /// Set the balance of an account `who` to some `amount`.
    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }

    /// Get the balance of an account `who`.
    /// If the account has no stored balance, we return zero.
    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }

    /// Transfer `amount` from one account to another.
    /// This function verifies that `from` has at least `amount` balance to transfer,
    /// and that no mathematical overflows occur.
    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> DispatchResult {
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

// A public enum which describes the calls we want to expose to the dispatcher.
// We should expect that the caller of each call will be provided by the dispatcher,
// and not included as a parameter of the call.
pub enum Call<T: Config> {
    Transfer {
        to: T::AccountId,
        amount: T::Balance,
    },
}

/// Implementation of the dispatch logic, mapping from `BalancesCall` to the appropriate underlying
/// function we want to execute.
impl<T: Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::AccountId;
    type Call = Call<T>;

    fn dispatch(
        &mut self,
        caller: Self::Caller,
        call: Self::Call,
    ) -> crate::support::DispatchResult {
        match call {
            Call::Transfer { to, amount } => {
                self.transfer(caller, to, amount)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Pallet;

    struct TestConfig;

    impl crate::system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    impl super::Config for TestConfig {
        type Balance = u128;
    }

    #[test]
    fn init_balances() {
        let mut balances = Pallet::<TestConfig>::new();

        assert_eq!(balances.balance(&"alice".to_string()), 0);

        balances.set_balance(&"alice".to_string(), 100);

        assert_eq!(balances.balance(&"alice".to_string()), 100);
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut balances = Pallet::<TestConfig>::new();

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
