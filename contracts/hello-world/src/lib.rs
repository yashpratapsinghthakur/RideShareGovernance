#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Env, Symbol,
};

const GOV_NS: Symbol = symbol_short!("RSGOV");

#[contracttype]
#[derive(Clone)]
pub enum ProposalStatus {
    Active,
    Closed,
}

#[contracttype]
#[derive(Clone)]
pub struct Proposal {
    pub proposal_id: u64,
    pub creator: Symbol,
    pub title: Symbol,
    pub yes_votes: i128,
    pub no_votes: i128,
    pub status: ProposalStatus,
}

#[contract]
pub struct RideShareGovernance;

#[contractimpl]
impl RideShareGovernance {
    pub fn create_proposal(env: Env, proposal_id: u64, creator: Symbol, title: Symbol) {
        let key = Self::proposal_key(proposal_id);
        let inst = env.storage().instance();

        if inst.has(&key) {
            panic!("proposal id exists");
        }

        let p = Proposal {
            proposal_id,
            creator,
            title,
            yes_votes: 0,
            no_votes: 0,
            status: ProposalStatus::Active,
        };
        inst.set(&key, &p);
    }

    pub fn vote(env: Env, proposal_id: u64, support: bool, weight: i128) {
        if weight <= 0 {
            panic!("weight must be positive");
        }

        let key = Self::proposal_key(proposal_id);
        let inst = env.storage().instance();

        let mut p: Proposal = inst.get(&key).unwrap_or_else(|| panic!("proposal not found"));

        if let ProposalStatus::Active = p.status {
        } else {
            panic!("proposal closed");
        }

        if support {
            p.yes_votes += weight;
        } else {
            p.no_votes += weight;
        }

        inst.set(&key, &p);
    }

    pub fn close_proposal(env: Env, proposal_id: u64) {
        let key = Self::proposal_key(proposal_id);
        let inst = env.storage().instance();

        let mut p: Proposal = inst.get(&key).unwrap_or_else(|| panic!("proposal not found"));
        p.status = ProposalStatus::Closed;
        inst.set(&key, &p);
    }

    pub fn is_passed(env: Env, proposal_id: u64) -> bool {
        let key = Self::proposal_key(proposal_id);
        let inst = env.storage().instance();

        let p: Option<Proposal> = inst.get(&key);
        match p {
            Some(pp) => {
                if let ProposalStatus::Closed = pp.status {
                    pp.yes_votes > pp.no_votes
                } else {
                    false
                }
            }
            None => false,
        }
    }

    pub fn get_proposal(env: Env, proposal_id: u64) -> Option<Proposal> {
        let key = Self::proposal_key(proposal_id);
        let inst = env.storage().instance();
        inst.get(&key)
    }

    fn proposal_key(id: u64) -> (Symbol, u64) {
        (GOV_NS, id)
    }
}
