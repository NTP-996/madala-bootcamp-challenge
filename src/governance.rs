use crate::staking::StakingConfig;
use std::collections::HashMap;

// Proposal Status Enum
#[derive(Clone)]
pub enum ProposalStatus {
    Active,
    Approved,
    Rejected,
}

// Proposal Struct
pub struct Proposal {
    pub description: String,
    pub yes_votes: u32,
    pub no_votes: u32,
    pub status: ProposalStatus,
}

// Governance Trait
pub trait GovernanceConfig: StakingConfig {}

pub struct GovernancePallet<T: GovernanceConfig> {
    proposals: HashMap<u32, Proposal>,
    votes: HashMap<(T::AccountId, u32), bool>,
    next_proposal_id: u32,
}

impl<T: GovernanceConfig> GovernancePallet<T> {
    pub fn new() -> Self {
        Self {
            proposals: HashMap::new(),
            votes: HashMap::new(),
            next_proposal_id: 0,
        }
    }

    // Create Proposal
    pub fn create_proposal(
        &mut self,
        _creator: T::AccountId,
        description: String,
    ) -> Result<u32, &'static str> {
        let proposal_id = self.next_proposal_id;
        self.next_proposal_id += 1;

        self.proposals.insert(
            proposal_id,
            Proposal {
                description,
                yes_votes: 0,
                no_votes: 0,
                status: ProposalStatus::Active,
            },
        );

        Ok(proposal_id)
    }

    // Vote on Proposal
    pub fn vote_on_proposal(
        &mut self,
        voter: T::AccountId,
        proposal_id: u32,
        vote_type: bool,
    ) -> Result<(), &'static str> {
        if self.votes.contains_key(&(voter.clone(), proposal_id)) {
            return Err("You can only vote once");
        }

        match self.proposals.get_mut(&proposal_id) {
            Some(proposal) => {
                self.votes.insert((voter, proposal_id), vote_type);

                if vote_type {
                    proposal.yes_votes += 1;
                } else {
                    proposal.no_votes += 1;
                }

                Ok(())
            }
            None => Err("Proposal not found"),
        }
    }

    // Get Proposal
    pub fn get_proposal(&self, proposal_id: u32) -> Option<&Proposal> {
        self.proposals.get(&proposal_id)
    }

    // Finalize Proposal
    pub fn finalize_proposal(&mut self, proposal_id: u32) -> Result<ProposalStatus, &'static str> {
        match self.proposals.get_mut(&proposal_id) {
            Some(proposal) => {
                proposal.status = if proposal.yes_votes > proposal.no_votes {
                    ProposalStatus::Approved
                } else {
                    ProposalStatus::Rejected
                };

                Ok(proposal.status.clone())
            }
            None => Err("Proposal not found"),
        }
    }
}