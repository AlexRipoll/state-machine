use std::{
    collections::BTreeMap,
    ops::{Add, AddAssign},
};

use num::{CheckedAdd, One, Zero};

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<AccountId, BlockNumber, Nonce> {
    /// The current block number.
    block_number: BlockNumber,
    /// A map from an account to their nonce.
    nonce: BTreeMap<AccountId, Nonce>,
}

impl<AccountId, BlockNumber, Nonce> Pallet<AccountId, BlockNumber, Nonce>
where
    AccountId: Ord + Clone,
    BlockNumber: Zero + One + CheckedAdd + AddAssign + Copy,
    Nonce: Zero + One + CheckedAdd + Copy,
{
    /// Create a new instance of the System Pallet.
    pub fn new() -> Self {
        Self {
            block_number: BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    /// Get the current block number.
    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    // This function can be used to increment the block number.
    // Increases the block number by one.
    pub fn inc_block_number(&mut self) {
        self.block_number += BlockNumber::one();
    }

    // Increment the nonce of an account. This helps us keep track of how many transactions each
    // account has made.
    pub fn inc_nonce(&mut self, who: &AccountId) {
        let binding = Nonce::zero();

        let nonce = self.nonce.get(who).unwrap_or(&binding);
        self.nonce.insert(who.clone(), nonce.add(Nonce::one()));
    }
}

#[cfg(test)]
mod test {
    use super::Pallet;

    #[test]
    fn init_system() {
        let mut system = Pallet::<String, u32, u32>::new();

        system.inc_block_number();
        assert_eq!(system.block_number, 1);

        system.inc_nonce(&"alice".to_string());
        assert_eq!(system.nonce.get(&"alice".to_string()).unwrap(), &1);
    }
}
