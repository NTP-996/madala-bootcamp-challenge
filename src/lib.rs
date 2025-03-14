pub use governance::GovernanceConfig;
pub use staking::StakingConfig;
pub use system::SystemConfig;

pub mod governance;
pub mod staking;
pub mod system;

pub struct Runtime;

// Implement specific System configuration for the runtime
impl SystemConfig for Runtime {
    type AccountId = u64;
}

// Implement specific Staking configuration for the runtime
impl StakingConfig for Runtime {
    type Balance = u64;
}

// Implement specific Governance configuration for the runtime
impl GovernanceConfig for Runtime {
    // No additional types needed at this level
}