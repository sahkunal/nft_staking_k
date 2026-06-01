# NFT Staking Program (MPL Core + Anchor)

A Solana NFT staking protocol built with Anchor and MPL Core.

Users can:

* Create an MPL Core collection
* Mint NFTs into the collection
* Stake NFTs
* Freeze staked NFTs
* Earn reward tokens based on staking duration
* Unstake NFTs after a configurable freeze period
* Automatically mint rewards upon unstaking

---

# Features

## Collection Creation

Creates an MPL Core collection whose update authority is a PDA owned by the staking program.

PDA:

```text
update_authority
├── seed: "update_authority"
├── collection pubkey
└── bump
```

This PDA becomes the collection update authority and is later used for staking operations.

---

## NFT Minting

NFTs are minted into the collection using MPL Core.

Each NFT:

* Belongs to the collection
* Is owned by the user
* Inherits collection authority relationships

---

## Config Initialization

Each collection has its own staking configuration.

Config PDA:

```text
config
├── seed: "config"
├── collection pubkey
└── bump
```

Stores:

```rust
pub struct Config {
    pub rewards_bps: u16,
    pub freeze_period: u16,
    pub rewards_bump: u8,
    pub bumps: u8,
}
```

---

## Reward Mint

A reward token mint is created during initialization.

PDA:

```text
rewards_mint
├── seed: "rewards_mint"
├── config pubkey
└── bump
```

Properties:

```text
Decimals: 6
Authority: Config PDA
```

The staking program mints rewards directly from this mint.

---

# Reward Formula

Rewards are calculated during unstaking.

```text
reward =
(staking_days × rewards_bps × 10^decimals)
/ 10000
```

Where:

```text
rewards_bps = basis points

10000 = 100%
5000  = 50%
1000  = 10%
100   = 1%
```

Example:

```text
rewards_bps = 10000
staking_days = 7

reward = 7 tokens
```

---

# Staking Process

When a user stakes an NFT:

1. NFT ownership is verified
2. Collection authority is verified
3. Attributes plugin is added/updated
4. NFT is frozen

Attributes added:

```text
staked = true
staked_at = <unix_timestamp>
```

Freeze Plugin:

```text
FreezeDelegate {
    frozen: true
}
```

This prevents NFT transfers while staked.

---

# Unstaking Process

When a user unstakes:

1. Verify NFT is staked
2. Read staking timestamp
3. Verify freeze period elapsed
4. Calculate rewards
5. Mint reward tokens
6. Update attributes
7. Unfreeze NFT

Attributes updated:

```text
staked = false
staked_at = <original_timestamp>
```

Freeze Plugin updated:

```text
FreezeDelegate {
    frozen: false
}
```

---

# Program Instructions

## create_collection

Creates a new MPL Core collection.

Parameters:

```rust
name: String
uri: String
```

Creates:

```text
Collection Account
Update Authority PDA
```

---

## mint_asset

Mints an NFT into a collection.

Parameters:

```rust
name: String
uri: String
```

Creates:

```text
MPL Core Asset
```

---

## initialize

Initializes staking configuration.

Parameters:

```rust
rewards_bps: u16
freeze_period: u16
```

Creates:

```text
Config PDA
Rewards Mint PDA
```

---

## stake

Stake an NFT.

Checks:

```text
Owner verification
Collection verification
Update authority verification
Already staked prevention
```

Effects:

```text
Add staking metadata
Freeze NFT
```

---

## unstake

Unstake NFT and claim rewards.

Checks:

```text
NFT is staked
Freeze period elapsed
Valid staking metadata
```

Effects:

```text
Mint rewards
Update metadata
Unfreeze NFT
```

---

# Program Derived Addresses

## Update Authority PDA

```text
Seeds:
[
    "update_authority",
    collection.key()
]
```

Purpose:

```text
Collection update authority
Plugin authority
Freeze authority
```

---

## Config PDA

```text
Seeds:
[
    "config",
    collection.key()
]
```

Purpose:

```text
Collection staking configuration
```

---

## Rewards Mint PDA

```text
Seeds:
[
    "rewards_mint",
    config.key()
]
```

Purpose:

```text
Mint reward tokens
```

---

# Errors

## InvalidOwner

```text
NFT owner does not match signer
```

---

## InvalidUpdateAuthority

```text
Collection authority mismatch
```

---

## AlreadyStaked

```text
NFT already staked
```

---

## AssetNotStaked

```text
NFT is not currently staked
```

---

## InvalidTimestamp

```text
Staking timestamp missing or invalid
```

---

## FreezePeriodNotElapsed

```text
User attempted unstake before freeze period ended
```

---

## InvalidRewardsBps

```text
Invalid reward basis points value
```

---

# Architecture

```text
User
 │
 ▼
Collection
 │
 ▼
Update Authority PDA
 │
 ├───────────────┐
 ▼               ▼
Stake NFT     Unstake NFT
 │               │
 ▼               ▼
Freeze NFT   Mint Rewards
 │               │
 ▼               ▼
Attributes   Rewards ATA
```

---

# Tech Stack

* Solana
* Anchor 0.31.1
* MPL Core 0.11.2
* Anchor SPL
* TypeScript Tests
* LiteSVM (Rust Testing)

---

# Build

```bash
anchor build
```

---

# Test

```bash
anchor test
```

or

```bash
anchor test --skip-build
```

---

# Program ID

```text
3JwmfbxuaZnTzkYiFnpJYyfTfi9LCmMXdASyvyhJaUnc
```
