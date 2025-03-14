use crate::system::SystemConfig;
use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::HashMap;

pub trait StakingConfig: SystemConfig {
    // Define the Balance type with ability to perform checked arithmetic operations
    type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

pub struct StakingPallet<T: StakingConfig> {
    // Track free balances for each account
    pub free_balances: HashMap<T::AccountId, T::Balance>,
    // Track staked balances for each account
    pub staked_balances: HashMap<T::AccountId, T::Balance>,
}

impl<T: StakingConfig> StakingPallet<T> {
    pub fn new() -> Self {
        Self {
            free_balances: HashMap::default(),
            staked_balances: HashMap::default(),
        }
    }

    // Set free balance for an account
    pub fn set_balance(&mut self, who: T::AccountId, amount: T::Balance) {
        self.free_balances.insert(who.clone(), amount);
    }

    // Stake tokens (move from free to staked)
    pub fn stake(&mut self, who: T::AccountId, amount: T::Balance) -> Result<(), &'static str> {
        let available_balance = self.get_free_balance(who.clone());
        let current_stake = self.get_staked_balance(who.clone());

        let new_free_balance = available_balance.checked_sub(&amount).ok_or("not enough funds")?;
        let new_stake_balance = current_stake.checked_add(&amount).ok_or("overflow")?;

        self.free_balances.insert(who.clone(), new_free_balance);
        self.staked_balances.insert(who.clone(), new_stake_balance);

        Ok(())
    }

    // Unstake tokens (move from staked to free)
    pub fn unstake(&mut self, who: T::AccountId, amount: T::Balance) -> Result<(), &'static str> {
        let current_stake = self.get_staked_balance(who.clone());
        let available_balance = self.get_free_balance(who.clone());

        let new_stake_balance = current_stake.checked_sub(&amount).ok_or("not enough funds")?;
        let new_free_balance = available_balance.checked_add(&amount).ok_or("overflow")?;

        self.free_balances.insert(who.clone(), new_free_balance);
        self.staked_balances.insert(who.clone(), new_stake_balance);

        Ok(())
    }

    // Get free balance for an account
    pub fn get_free_balance(&self, who: T::AccountId) -> T::Balance {
        self.free_balances.get(&who).copied().unwrap_or_else(T::Balance::zero)
    }

     // Get staked balance for an account
    pub fn get_staked_balance(&self, who: T::AccountId) -> T::Balance {
        self.staked_balances.get(&who).copied().unwrap_or_else(T::Balance::zero)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Runtime;

    #[test]
    fn test_staking_should_work() {
        let alice = 1u64;
        let mut staking = StakingPallet::<Runtime>::new();

        staking.set_balance(alice, 1000);

        assert_eq!(staking.get_free_balance(alice), 1000);
        assert_eq!(staking.get_staked_balance(alice), 0);

        assert!(staking.stake(alice, 400).is_ok());
        assert_eq!(staking.get_free_balance(alice), 600);
        assert_eq!(staking.get_staked_balance(alice), 400);

        assert!(staking.unstake(alice, 100).is_ok());
        assert_eq!(staking.get_free_balance(alice), 700);
        assert_eq!(staking.get_staked_balance(alice), 300);
    }

    #[test]
    fn test_staking_errors() {
        let bob = 2u64;
        let mut staking = StakingPallet::<Runtime>::new();

        staking.set_balance(bob, 500);

        assert!(staking.stake(bob, 600).is_err());
        assert!(staking.stake(bob, 300).is_ok());
        assert!(staking.unstake(bob, 400).is_err());
    }
}