#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol, Vec,
};

const BADGE_NS: Symbol = symbol_short!("SBADGE");

#[contracttype]
#[derive(Clone)]
pub enum BadgeStatus {
    Active,
    Revoked,
}

#[contracttype]
#[derive(Clone)]
pub struct SkillBadge {
    pub badge_id: u64,
    pub holder: Address,
    pub issuer: Address,
    pub skill_name: String,
    pub level: u32,
    pub status: BadgeStatus,
    pub endorsements: u32,
}

#[contract]
pub struct SkillEndorseBadge;

#[contractimpl]
impl SkillEndorseBadge {
    // Issue a new skill badge to a holder
    pub fn issue_badge(
        env: Env,
        badge_id: u64,
        holder: Address,
        issuer: Address,
        skill_name: String,
        level: u32,
    ) {
        let inst = env.storage().instance();
        let key = Self::badge_key(badge_id);

        if inst.has(&key) {
            panic!("badge_id exists");
        }

        let badge = SkillBadge {
            badge_id,
            holder,
            issuer,
            skill_name,
            level,
            status: BadgeStatus::Active,
            endorsements: 0,
        };

        inst.set(&key, &badge);
    }

    // Endorser endorses an active badge
    pub fn endorse_badge(env: Env, badge_id: u64, _endorser: Address) {
        let inst = env.storage().instance();
        let key = Self::badge_key(badge_id);

        let mut badge: SkillBadge =
            inst.get(&key).unwrap_or_else(|| panic!("badge not found"));

        if let BadgeStatus::Active = badge.status {
        } else {
            panic!("badge not active");
        }

        badge.endorsements = badge
            .endorsements
            .checked_add(1)
            .unwrap_or_else(|| panic!("endorsement overflow"));

        inst.set(&key, &badge);
    }

    // Issuer can revoke a badge
    pub fn revoke_badge(env: Env, badge_id: u64, caller: Address) {
        let inst = env.storage().instance();
        let key = Self::badge_key(badge_id);

        let mut badge: SkillBadge =
            inst.get(&key).unwrap_or_else(|| panic!("badge not found"));

        if caller != badge.issuer {
            panic!("only issuer can revoke");
        }

        badge.status = BadgeStatus::Revoked;
        inst.set(&key, &badge);
    }

    // Read badge details
    pub fn get_badge(env: Env, badge_id: u64) -> Option<SkillBadge> {
        let inst = env.storage().instance();
        let key = Self::badge_key(badge_id);
        inst.get(&key)
    }

    // Internal key helper
    fn badge_key(id: u64) -> (Symbol, u64) {
        (BADGE_NS, id)
    }
}
