use crate::staking::StakingConfig;
// use crate::system::SystemConfig;
use std::collections::HashMap;

pub trait GovernanceConfig: StakingConfig {}

pub struct Proposal {
    description: String,
    yes_votes: u32,
    no_votes: u32,
    status: ProposalStatus,
    
}

#[derive(Clone)]
pub enum ProposalStatus {
    Active,
    Approved,
    Rejected,
}

impl Proposal {
    pub fn description(&self) -> &str {
        &self.description
    }
}

pub struct GovernancePallet<T: GovernanceConfig> {
    pub proposals: HashMap<u32, Proposal>,
    pub votes: HashMap<(T::AccountId, u32), bool>, // (voter, proposal_id) -> vote_type
    next_proposal_id: u32,
}

impl<T: GovernanceConfig> GovernancePallet<T> {
    pub fn new() -> Self {
        // todo!()
        // create new struct for GovernancePallet
        Self {
            proposals: HashMap::new(),
            votes: HashMap::new(),
            next_proposal_id: 0,
        }
    }

    // Create a new proposal
    pub fn create_proposal(
        &mut self,
        _creator: T::AccountId,
        description: String,
    ) -> Result<u32, &'static str> {
        // todo!()
        // create a new proposal and insert it into the proposals hashmap with the next_proposal_id as the key
        let proposal_id = self.next_proposal_id;
        let proposal = Proposal {
            description,
            yes_votes: 0,
            no_votes: 0,
            status: ProposalStatus::Active,
        };

        self.proposals.insert(proposal_id, proposal);
        self.next_proposal_id += 1;

        Ok(proposal_id)
    }

    // Vote on a proposal (true = yes, false = no)
    pub fn vote(
        &mut self,
        voter: T::AccountId,
        proposal_id: u32,
        vote_type: bool,
    ) -> Result<(), &'static str> {
        // todo!()
        // check if the proposal exists in the proposals hashmap using the proposal_id
        // if the voter has already voted, return an error
        // if the voter has not voted, increment the yes_votes or no_votes count based on the vote_type
        // insert the vote into the votes hashmap
        if let Some(proposal) = self.proposals.get_mut(&proposal_id) {
            if let Some(vote) = self.votes.get(&(voter.clone(), proposal_id)) {
                if *vote == vote_type {
                    return Err("You have already voted with the same vote type");
                }
            }

            if vote_type {
                proposal.yes_votes += 1;
            } else {
                proposal.no_votes += 1;
            }

            self.votes.insert((voter.clone(), proposal_id), vote_type);
            Ok(())
        } else {
            Err("Proposal does not exist")
        }
    }

    // Get proposal details
    pub fn get_proposal(&self, proposal_id: u32) -> Option<&Proposal> {
        // todo!()
        // return the proposal details from the proposals hashmap based on the proposal_id
        self.proposals.get(&proposal_id)
    }

    // Finalize a proposal (changes status based on votes)
    pub fn finalize_proposal(&mut self, proposal_id: u32) -> Result<ProposalStatus, &'static str> {
        // todo!() 
        // check if the proposal exists in the proposals hashmap using the proposal_id
        // if the proposal has more yes votes than no votes, change the status to Approved
        // if the proposal has more no votes than yes votes, change the status to Rejected
        // return the final status of the proposal
        if let Some(proposal) = self.proposals.get_mut(&proposal_id) {
            if proposal.yes_votes > proposal.no_votes {
                proposal.status = ProposalStatus::Approved;
            } else {
                proposal.status = ProposalStatus::Rejected;
            }

            Ok(proposal.status.clone())
        } else {
            Err("Proposal does not exist")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Runtime;

    #[test]
    fn test_governance_should_work() {
        let alice = 1u64;
        let bob = 2u64;
        let charlie = 3u64;

        let mut governance = GovernancePallet::<Runtime>::new();

        // Create a proposal
        let proposal_id = governance
            .create_proposal(alice, "Increase validator rewards".to_string())
            .unwrap();

        // Cast votes
        governance.vote(alice, proposal_id, true).unwrap(); // Yes vote
        governance.vote(bob, proposal_id, true).unwrap(); // Yes vote
        governance.vote(charlie, proposal_id, false).unwrap(); // No vote

        // Check proposal status before finalization
        let proposal = governance.get_proposal(proposal_id).unwrap();
        assert_eq!(proposal.yes_votes, 2);
        assert_eq!(proposal.no_votes, 1);

        // Finalize proposal
        let status = governance.finalize_proposal(proposal_id).unwrap();
        assert!(matches!(status, ProposalStatus::Approved));

        // Check proposal is now approved
        let finalized_proposal = governance.get_proposal(proposal_id).unwrap();
        assert!(matches!(
            finalized_proposal.status,
            ProposalStatus::Approved
        ));
    }
}
