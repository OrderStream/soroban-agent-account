# soroban-agent-account ?????

[![Crates.io](https://img.shields.io/crates/v/soroban-agent-account.svg)](https://crates.io/crates/soroban-agent-account)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Soroban SDK](https://img.shields.io/badge/Soroban%20SDK-v22.0.0-purple.svg)](https://stellar.org/soroban)
[![Stellar Protocol](https://img.shields.io/badge/Stellar-Protocol%2022-brightgreen.svg)](https://stellar.org)

**Production Autonomous AI Agent Smart Account & Policy-Enforced Spending Wallet for Stellar Soroban.**

Enables AI agents (AutoGPT, LangChain, Claude Code, ElizaOS, OpenHands) to execute on-chain transactions and payments autonomously under cryptographically enforced on-chain guardrails.

---

## ?? The Agentic Web3 Crisis

When giving an autonomous AI coding or trading agent a Web3 wallet, teams face severe risks:
1. **Unbounded Financial Drain**: An agent caught in a recursive loop or hallucinating a bad trade can drain the entire wallet balance in seconds.
2. **Untrusted Target Invocations**: An indirect prompt injection can trick an agent into sending treasury funds to an attacker's smart contract.
3. **Session Key Compromise**: Long-lived private keys stored in agent memory files can be exfiltrated.
4. **Storage TTL Eviction Freeze**: Protocol 22 storage decay causes idle agent smart accounts to expire and lock up.

**`soroban-agent-account` solves this on-chain.** The contract acts as a custom smart account (`CustomAccountInterface`), enforcing strict programmatic spending envelopes before any transaction is authorized.

---

## ??? Core Capabilities

| Guardrail | Enforcement Level | Mechanism |
|---|---|---|
| **Per-Transaction Cap** | On-Chain (`__check_auth`) | Rejects any transfer exceeding `per_tx_limit` (e.g. max 50 USDC per action). |
| **Rolling Daily Budget** | On-Chain (`__check_auth`) | Tracks cumulative spend across a 17,280 ledger rolling window (~24 hours). |
| **Target Whitelisting** | On-Chain (`__check_auth`) | Restricts the agent to interact only with approved DEX, bridge, or oracle contracts. |
| **Session Key TTL** | On-Chain (`__check_auth`) | Agent session keys expire automatically at `session_expires_at` ledger unless renewed by human admin. |
| **Emergency Killswitch** | Human Admin Override | Human admin can call `set_paused(true)` to instantly freeze all agent actions. |
| **Auto-TTL Extension** | Automated Rent Guard | Every authorized invocation extends instance and state TTL to prevent archival eviction. |

---

## ?? Quickstart

### 1. Add to `Cargo.toml`
```toml
[dependencies]
soroban-agent-account = "0.1.0"
soroban-sdk = "22.0.0"
```

### 2. Initialize Agent Spending Policy
```rust
use soroban_sdk::{Address, BytesN, Env};
use soroban_agent_account::{AgentAccountContract, AgentAccountContractClient};

let env = Env::default();
let contract_id = env.register(AgentAccountContract, ());
let client = AgentAccountContractClient::new(&env, &contract_id);

// Admin initializes account with:
// - Agent session public key
// - Session duration: 10,000 ledgers (~14 hours)
// - Per-tx limit: 50 USDC (50_0000000 stroops)
// - Daily cap: 500 USDC (500_0000000 stroops)
client.init(
    &admin_address,
    &agent_public_key,
    &10_000u32,
    &50_0000000i128,
    &500_0000000i128
);

// Whitelist approved DEX contract
client.add_allowed_contract(&admin_address, &dex_contract_address);
```

---

## ?? Direct Treasury & Sponsorship
OrderStream commercial development is supported directly on Stellar:
* **Stellar (USDC / XLM)**: `GABFQIK63R2NETJM7T673EAMZN4RJLLGP3OFUEJU5ZVTGWUKULZJNL6` *(Memo: `400566831`)*
* **EVM Base (USDC)**: `0x5de2b62434754bfd39ef350a3a867ed3eced5074`
