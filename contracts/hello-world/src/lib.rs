#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Address, Env, Symbol,
};

#[contract]
pub struct StudyReputation;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Reputation(Address),
    Challenge(Address),
    Claim(Address),
    Votes(Address),
    Streak(Address),
}

#[contractimpl]
impl StudyReputation {

    // Tạo challenge mới
    pub fn create_challenge(
        env: Env,
        user: Address,
        challenge: Symbol,
    ) {
        user.require_auth();

        env.storage()
            .instance()
            .set(&DataKey::Challenge(user), &challenge);
    }

    // Submit claim hoàn thành
    pub fn submit_claim(
        env: Env,
        user: Address,
    ) {
        user.require_auth();

        env.storage()
            .instance()
            .set(&DataKey::Claim(user.clone()), &true);

        env.storage()
            .instance()
            .set(&DataKey::Votes(user), &0u32);
    }

    // Peer xác thực
    pub fn verify_claim(
        env: Env,
        verifier: Address,
        target: Address,
    ) {
        verifier.require_auth();

        let claimed: bool = env.storage()
            .instance()
            .get(&DataKey::Claim(target.clone()))
            .unwrap_or(false);

        if !claimed {
            panic!("No active claim");
        }

        let votes: u32 = env.storage()
            .instance()
            .get(&DataKey::Votes(target.clone()))
            .unwrap_or(0);

        let new_votes = votes + 1;

        env.storage()
            .instance()
            .set(&DataKey::Votes(target.clone()), &new_votes);

        // đủ 3 vote -> tăng reputation
        if new_votes >= 3 {
            let rep: u32 = env.storage()
                .instance()
                .get(&DataKey::Reputation(target.clone()))
                .unwrap_or(0);

            env.storage()
                .instance()
                .set(&DataKey::Reputation(target.clone()), &(rep + 10));

            let streak: u32 = env.storage()
                .instance()
                .get(&DataKey::Streak(target.clone()))
                .unwrap_or(0);

            env.storage()
                .instance()
                .set(&DataKey::Streak(target.clone()), &(streak + 1));

            env.storage()
                .instance()
                .set(&DataKey::Claim(target), &false);
        }
    }

    // Xem reputation
    pub fn get_reputation(
        env: Env,
        user: Address,
    ) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::Reputation(user))
            .unwrap_or(0)
    }

    // Xem streak
    pub fn get_streak(
        env: Env,
        user: Address,
    ) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::Streak(user))
            .unwrap_or(0)
            }

    // Xem challenge hiện tại
    pub fn get_challenge(
        env: Env,
        user: Address,
    ) -> Symbol {
        env.storage()
            .instance()
            .get(&DataKey::Challenge(user))
            .unwrap_or(symbol_short!("NONE"))
    }
}