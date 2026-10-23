#![no_std]

use soroban_sdk::{
    auth::{Context, CustomAccountInterface},
    contract, contracterror, contractimpl, contracttype,
    crypto::Hash, Address, Bytes, BytesN, Env, Symbol, TryFromVal, Vec,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum AgentAccountError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    UnauthorizedAdmin = 3,
    AccountPaused = 4,
    AgentSessionExpired = 5,
    InvalidSignature = 6,
    TargetContractNotAllowed = 7,
    PerTxSpendingLimitExceeded = 8,
    DailySpendingLimitExceeded = 9,
    InvalidPolicyConfig = 10,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    AgentPublicKey,
    SessionExpiresAt,
    IsPaused,
    PerTxLimit,
    DailyLimit,
    SpentInCurrentWindow,
    WindowStartLedger,
    AllowedContracts,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct AgentPolicyStatus {
    pub is_paused: bool,
    pub session_expires_at: u32,
    pub current_ledger: u32,
    pub per_tx_limit: i128,
    pub daily_limit: i128,
    pub spent_in_window: i128,
}

#[contract]
pub struct AgentAccountContract;

#[contractimpl]
impl AgentAccountContract {
    /// Initialize the Agent Smart Account with human admin and initial agent spending boundaries.
    pub fn init(
        env: Env,
        admin: Address,
        agent_public_key: BytesN<32>,
        session_duration_ledgers: u32,
        per_tx_limit: i128,
        daily_limit: i128,
    ) -> Result<(), AgentAccountError> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(AgentAccountError::AlreadyInitialized);
        }

        if per_tx_limit <= 0 || daily_limit < per_tx_limit {
            return Err(AgentAccountError::InvalidPolicyConfig);
        }

        let current_ledger = env.ledger().sequence();
        let expires_at = current_ledger.saturating_add(session_duration_ledgers);

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::AgentPublicKey, &agent_public_key);
        env.storage().instance().set(&DataKey::SessionExpiresAt, &expires_at);
        env.storage().instance().set(&DataKey::IsPaused, &false);
        env.storage().instance().set(&DataKey::PerTxLimit, &per_tx_limit);
        env.storage().instance().set(&DataKey::DailyLimit, &daily_limit);
        env.storage().instance().set(&DataKey::SpentInCurrentWindow, &0i128);
        env.storage().instance().set(&DataKey::WindowStartLedger, &current_ledger);
        
        let empty_allowed: Vec<Address> = Vec::new(&env);
        env.storage().instance().set(&DataKey::AllowedContracts, &empty_allowed);

        // Proactive TTL extension preventing archival eviction
        env.storage().instance().extend_ttl(50_000, 100_000);
        Ok(())
    }

    /// Whitelist a specific contract that the AI Agent is authorized to interact with.
    pub fn add_allowed_contract(env: Env, admin: Address, target_contract: Address) -> Result<(), AgentAccountError> {
        admin.require_auth();
        let stored_admin: Address = env.storage().instance().get(&DataKey::Admin).ok_or(AgentAccountError::NotInitialized)?;
        if admin != stored_admin {
            return Err(AgentAccountError::UnauthorizedAdmin);
        }

        let mut allowed: Vec<Address> = env.storage().instance().get(&DataKey::AllowedContracts).unwrap_or(Vec::new(&env));
        if !allowed.contains(&target_contract) {
            allowed.push_back(target_contract);
            env.storage().instance().set(&DataKey::AllowedContracts, &allowed);
        }
        env.storage().instance().extend_ttl(50_000, 100_000);
        Ok(())
    }

    /// Emergency pause switch: instantly halt all AI agent execution.
    pub fn set_paused(env: Env, admin: Address, paused: bool) -> Result<(), AgentAccountError> {
        admin.require_auth();
        let stored_admin: Address = env.storage().instance().get(&DataKey::Admin).ok_or(AgentAccountError::NotInitialized)?;
        if admin != stored_admin {
            return Err(AgentAccountError::UnauthorizedAdmin);
        }

        env.storage().instance().set(&DataKey::IsPaused, &paused);
        env.storage().instance().extend_ttl(50_000, 100_000);
        Ok(())
    }

    /// Rotate or renew the agent session public key and expiration duration.
    pub fn renew_session(
        env: Env,
        admin: Address,
        new_agent_key: BytesN<32>,
        duration_ledgers: u32,
    ) -> Result<(), AgentAccountError> {
        admin.require_auth();
        let stored_admin: Address = env.storage().instance().get(&DataKey::Admin).ok_or(AgentAccountError::NotInitialized)?;
        if admin != stored_admin {
            return Err(AgentAccountError::UnauthorizedAdmin);
        }

        let expires_at = env.ledger().sequence().saturating_add(duration_ledgers);
        env.storage().instance().set(&DataKey::AgentPublicKey, &new_agent_key);
        env.storage().instance().set(&DataKey::SessionExpiresAt, &expires_at);
        env.storage().instance().extend_ttl(50_000, 100_000);
        Ok(())
    }

    /// Returns the live spending envelope and policy status of the agent account.
    pub fn get_policy_status(env: Env) -> Result<AgentPolicyStatus, AgentAccountError> {
        let is_paused: bool = env.storage().instance().get(&DataKey::IsPaused).unwrap_or(false);
        let session_expires_at: u32 = env.storage().instance().get(&DataKey::SessionExpiresAt).unwrap_or(0);
        let per_tx_limit: i128 = env.storage().instance().get(&DataKey::PerTxLimit).unwrap_or(0);
        let daily_limit: i128 = env.storage().instance().get(&DataKey::DailyLimit).unwrap_or(0);
        let spent_in_window: i128 = env.storage().instance().get(&DataKey::SpentInCurrentWindow).unwrap_or(0);

        Ok(AgentPolicyStatus {
            is_paused,
            session_expires_at,
            current_ledger: env.ledger().sequence(),
            per_tx_limit,
            daily_limit,
            spent_in_window,
        })
    }
}

#[contractimpl]
impl CustomAccountInterface for AgentAccountContract {
    type Signature = BytesN<64>;
    type Error = AgentAccountError;

    /// Official Soroban Protocol 22 __check_auth implementation.
    /// Intercepts and validates all signature payloads, enforcing agent policy rules on-chain.
    fn __check_auth(
        env: Env,
        signature_payload: Hash<32>,
        signature: BytesN<64>,
        auth_contexts: Vec<Context>,
    ) -> Result<(), AgentAccountError> {
        // 1. Check if the smart account is paused
        let is_paused: bool = env.storage().instance().get(&DataKey::IsPaused).unwrap_or(false);
        if is_paused {
            return Err(AgentAccountError::AccountPaused);
        }

        // 2. Check if agent session has expired
        let current_ledger = env.ledger().sequence();
        let session_expires_at: u32 = env.storage().instance().get(&DataKey::SessionExpiresAt).unwrap_or(0);
        if current_ledger > session_expires_at {
            return Err(AgentAccountError::AgentSessionExpired);
        }

        // 3. Verify cryptographic Ed25519 signature
        let agent_pk: BytesN<32> = env.storage().instance().get(&DataKey::AgentPublicKey).ok_or(AgentAccountError::NotInitialized)?;
        let msg_bytes: Bytes = signature_payload.to_bytes().into();
        env.crypto().ed25519_verify(
            &agent_pk,
            &msg_bytes,
            &signature,
        );

        // 4. Enforce On-Chain Policy Rules over all authorization contexts
        let allowed_contracts: Vec<Address> = env.storage().instance().get(&DataKey::AllowedContracts).unwrap_or(Vec::new(&env));
        let per_tx_limit: i128 = env.storage().instance().get(&DataKey::PerTxLimit).unwrap_or(i128::MAX);
        let daily_limit: i128 = env.storage().instance().get(&DataKey::DailyLimit).unwrap_or(i128::MAX);
        
        // 17,280 ledgers ~= 24 hours at 5s/ledger
        const LEDGERS_PER_DAY: u32 = 17_280;
        let window_start: u32 = env.storage().instance().get(&DataKey::WindowStartLedger).unwrap_or(current_ledger);
        let mut spent_in_window: i128 = env.storage().instance().get(&DataKey::SpentInCurrentWindow).unwrap_or(0);

        // Reset rolling window if 24 hours elapsed
        if current_ledger.saturating_sub(window_start) >= LEDGERS_PER_DAY {
            spent_in_window = 0;
            env.storage().instance().set(&DataKey::WindowStartLedger, &current_ledger);
        }

        for context in auth_contexts.iter() {
            match context {
                Context::Contract(ctx) => {
                    // Check whitelist (if whitelist is non-empty)
                    if allowed_contracts.len() > 0 && !allowed_contracts.contains(&ctx.contract) {
                        return Err(AgentAccountError::TargetContractNotAllowed);
                    }

                    // Policy check on transfer/pay invocations
                    if ctx.fn_name == Symbol::new(&env, "transfer") || ctx.fn_name == Symbol::new(&env, "pay") {
                        // In transfer(from, to, amount), amount is the 3rd argument (index 2)
                        if ctx.args.len() >= 3 {
                            if let Some(amount_val) = ctx.args.get(2) {
                                if let Ok(amount) = i128::try_from_val(&env, &amount_val) {
                                    if amount > per_tx_limit {
                                        return Err(AgentAccountError::PerTxSpendingLimitExceeded);
                                    }
                                    if spent_in_window.saturating_add(amount) > daily_limit {
                                        return Err(AgentAccountError::DailySpendingLimitExceeded);
                                    }
                                    spent_in_window = spent_in_window.saturating_add(amount);
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        env.storage().instance().set(&DataKey::SpentInCurrentWindow, &spent_in_window);
        // Automatic TTL extension on every verified call
        env.storage().instance().extend_ttl(50_000, 100_000);
        Ok(())
    }
}
