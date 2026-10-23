#![cfg(test)]

use soroban_agent_account::{
    AgentAccountContract, AgentAccountContractClient,
};
use soroban_sdk::{
    testutils::{Address as _, BytesN as _},
    Address, BytesN, Env,
};

#[test]
fn test_init_and_policy_status() {
    let env = Env::default();
    let contract_id = env.register(AgentAccountContract, ());
    let client = AgentAccountContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let agent_pk = BytesN::from_array(&env, &[42u8; 32]);

    client.init(&admin, &agent_pk, &1000u32, &100i128, &500i128);

    let status = client.get_policy_status();
    assert_eq!(status.is_paused, false);
    assert_eq!(status.per_tx_limit, 100i128);
    assert_eq!(status.daily_limit, 500i128);
    assert_eq!(status.spent_in_window, 0i128);
}

#[test]
fn test_admin_whitelist_contract() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(AgentAccountContract, ());
    let client = AgentAccountContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let agent_pk = BytesN::from_array(&env, &[42u8; 32]);
    client.init(&admin, &agent_pk, &1000u32, &100i128, &500i128);

    let dex_contract = Address::generate(&env);
    client.add_allowed_contract(&admin, &dex_contract);
}

#[test]
fn test_emergency_pause_and_resume() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(AgentAccountContract, ());
    let client = AgentAccountContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let agent_pk = BytesN::from_array(&env, &[42u8; 32]);
    client.init(&admin, &agent_pk, &1000u32, &100i128, &500i128);

    client.set_paused(&admin, &true);
    let status = client.get_policy_status();
    assert_eq!(status.is_paused, true);

    client.set_paused(&admin, &false);
    let status2 = client.get_policy_status();
    assert_eq!(status2.is_paused, false);
}

#[test]
fn test_renew_session_key() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(AgentAccountContract, ());
    let client = AgentAccountContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let agent_pk1 = BytesN::from_array(&env, &[42u8; 32]);
    let agent_pk2 = BytesN::from_array(&env, &[99u8; 32]);

    client.init(&admin, &agent_pk1, &500u32, &100i128, &500i128);
    let s1 = client.get_policy_status();
    assert_eq!(s1.session_expires_at, 500u32);

    client.renew_session(&admin, &agent_pk2, &2000u32);
    let s2 = client.get_policy_status();
    assert_eq!(s2.session_expires_at, 2000u32);
}
