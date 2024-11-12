<h1 align="center">
  <code>p-token</code>
</h1>
<p align="center">
  <img width="400" alt="p-token" src="https://github.com/user-attachments/assets/ba1c5f0d-db2f-457d-8f7e-e62fd564e5e7" />
</p>
<p align="center">
  A <code>pinocchio</code>-based Token program.
</p>

<p align="center">
  <a href="https://github.com/febo/p-token/actions/workflows/main.yml"><img src="https://img.shields.io/github/actions/workflow/status/febo/p-token/main.yml?logo=GitHub" /></a>
</p>

## Overview

This repository contains a **proof-of-concept** of a reimplementation of the SPL Token program, one of the most used programs on Solana, using [`pinocchio`](https://github.com/febo/pinocchio). The purpose is to have an implementation that optimizes the compute units, while being fully compatible with the original implementation &mdash; i.e., support the exact same instruction and account layouts as SPL Token, byte for byte.

## Status

| Instruction                | Completed | CU (`p-token`) | CU (`spl-token`) |
|----------------------------|-----------|----------------|------------------|
| `InitializeMint`           | ✅        | 396            | 2967             |
| `InitializeAccount`        | ✅        | 444            | 4527             |
| `InitializeMultisig`       |           |                |                  |
| `Transfer`                 | ✅        | 161            | 4645             |
| `Approve`                  |           |                |                  |
| `Revoke`                   |           |                |                  |
| `SetAuthority`             |           |                |                  |
| `MintTo`                   | ✅        | 160            | 4538             |
| `Burn`                     |           |                |                  |
| `CloseAccount`             |           |                |                  |
| `FreezeAccount`            |           |                |                  |
| `ThawAccount`              |           |                |                  |
| `TransferChecked`          |           |                |                  |
| `ApproveChecked`           |           |                |                  |
| `MintToChecked`            |           |                |                  |
| `BurnChecked`              |           |                |                  |
| `InitializeAccount2`       |           |                |                  |
| `SyncNative`               |           |                |                  |
| `InitializeAccount3`       |           |                |                  |
| `InitializeMultisig2`      |           |                |                  |
| `InitializeMint2`          |           |                |                  |
| `GetAccountDataSize`       |           |                |                  |
| `InitializeImmutableOwner` |           |                |                  |
| `AmountToUiAmount`         |           |                |                  |
| `UiAmountToAmount`         |           |                |                  |

> Tests were run using Solana `v2.1.0`.

## Building

To build the programs from the root directory of the repository:
```bash
pnpm install
```
to install the required libraries, then:
```bash
pnpm programs:build
```

## Testing

To run the tests against both versions of the Token program:
```bash
pnpm programs:test
```
