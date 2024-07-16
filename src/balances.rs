use std::collections::BTreeMap;

#[derive(Debug)]
struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &String) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }

    /// Transfer `amount` from one account to another.
    /// This function verifies that `from` has at least `amount` balance to transfer,
    /// and that no mathematical overflows occur.
    pub fn transfer(
        &mut self,
        caller: String,
        to: String,
        amount: u128,
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_caller_balance = caller_balance
            .checked_sub(amount)
            .ok_or("insufficient balance")?;

        let new_to_balance = to_balance
            .checked_add(amount)
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
        let mut balances = Pallet::new();

        assert_eq!(balances.balance(&"alice".to_string()), 0);

        balances.set_balance(&"alice".to_string(), 100);

        assert_eq!(balances.balance(&"alice".to_string()), 100);
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }
    #[test]
    fn transfer_balance() {
        let mut balances = Pallet::new();

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
