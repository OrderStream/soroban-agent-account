# soroban-agent-account 🤖💳

[![Crates.io](https://img.shields.io/crates/v/soroban-agent-account.svg)](https://crates.io/crates/soroban-agent-account)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Soroban SDK](https://img.shields.io/badge/Soroban%20SDK-v22.0.0-purple.svg)](https://stellar.org/soroban)
[![Stellar Protocol](https://img.shields.io/badge/Stellar-Protocol%2022-brightgreen.svg)](https://stellar.org)
[![OrderStream Polar](https://img.shields.io/badge/Polar-OrderStream-blue.svg)](https://polar.sh/OrderStream)
[![Stellar XLM/USDC](https://img.shields.io/badge/Stellar-USDC%2FXLM-black.svg)](#direct-web3-settlement-wallets)
[![Base USDC](https://img.shields.io/badge/Base-USDC-0052FF.svg)](#direct-web3-settlement-wallets)

**Production Autonomous AI Agent Smart Account & Policy-Enforced Spending Wallet for Stellar Soroban.**

Enables AI agents (AutoGPT, LangChain, Claude Code, ElizaOS, OpenHands) to execute on-chain transactions and payments autonomously under cryptographically enforced on-chain guardrails.

---

## 🚨 The Agentic Web3 Crisis

When giving an autonomous AI coding or trading agent a Web3 wallet, teams face severe risks:
1. **Unbounded Financial Drain**: An agent caught in a recursive loop or hallucinating a bad trade can drain the entire wallet balance in seconds.
2. **Untrusted Target Invocations**: An indirect prompt injection can trick an agent into sending treasury funds to an attacker's smart contract.
3. **Session Key Compromise**: Long-lived private keys stored in agent memory files can be exfiltrated.
4. **Storage TTL Eviction Freeze**: Protocol 22 storage decay causes idle agent smart accounts to expire and lock up.

**`soroban-agent-account` solves this on-chain.** The contract acts as a custom smart account (`CustomAccountInterface`), enforcing strict programmatic spending envelopes before any transaction is authorized.

---

## 🏛️ Core Capabilities

| Guardrail | Enforcement Level | Mechanism |
| :--- | :--- | :--- |
| **Per-Transaction Cap** | On-Chain (`__check_auth`) | Rejects any transfer exceeding `per_tx_limit` (e.g. max 50 USDC per action). |
| **Rolling Daily Budget** | On-Chain (`__check_auth`) | Tracks cumulative spend across a 17,280 ledger rolling window (~24 hours). |
| **Target Whitelisting** | On-Chain (`__check_auth`) | Restricts the agent to interact only with approved DEX, bridge, or oracle contracts. |
| **Session Key TTL** | On-Chain (`__check_auth`) | Agent session keys expire automatically at `session_expires_at` ledger unless renewed by human admin. |
| **Emergency Killswitch** | Human Admin Override | Human admin can call `set_paused(true)` to instantly freeze all agent actions. |
| **Auto-TTL Extension** | Automated Rent Guard | Every authorized invocation extends instance and state TTL to prevent archival eviction. |

---

## 💳 Enterprise Pricing & Retainers

| Tier | Price | Scope | Included Features |
| :--- | :--- | :--- | :--- |
| **Open Source** | **$0** | Single Agents | Standalone smart account contract & CLI runner |
| **Starter CI Suite** | **$499 / mo** | 10 Repositories | Automated CI policy and daily limit checks |
| **Team Platform** | **$1,499 / mo** | 50 Repositories | All 10 OrderStream tools, 24/7 telemetry sentinel, anomaly alerts |
| **Enterprise Platform** | **$4,999 / mo** | Unlimited Agents | Multi-agent coordination vaults, custom HSM integrations, dedicated keeper |
| **Official Protocol Audit**| **$2,500 / audit**| Single Contract | Complete WASM opcode & spending envelope invariant verification |
| **M&A IP & Security Audit**| **$15,000 one-time**| Complete Protocol | Forensic code diligence, copyleft AST scan, and executive IP certificate |

---

## ⚡ Direct Web3 Settlement Wallets

OrderStream accepts direct crypto payments with **0% intermediary platform cuts**:

* **Stellar Protocol (USDC / XLM)**:  
  `GABFQIK63R2NETJM7T673EAMZN4RJLLGP3OFUEJU5ZVTGWUKULZJNL6`  
  *(Required Memo: `400566831`)*

* **EVM Base Network (USDC / ETH)**:  
  `0x5de2b62434754bfd39ef350a3a867ed3eced5074`

* **Corporate Credit Card & Recurring Subscriptions**:  
  Subscribe online via [Polar.sh/OrderStream](https://polar.sh/OrderStream).

---

## 🚀 CLI Runner

Validate agent spending rules and policy envelopes:

```bash
# Build binary
cargo build --release --bin soroban_agent_cli

# Validate agent spending limits and policies
./target/release/soroban_agent_cli validate
```

---

## 🛡️ GitHub Action Integration

Verify agent account spending policies in CI:

```yaml
- name: Verify AI Agent Spending Policies
  uses: OrderStream/soroban-agent-account@master
```

---

## 🧪 Verification & Testing

```bash
cargo test
```
