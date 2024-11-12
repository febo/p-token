# <img height="70" alt="tokenocchio" src="https://github.com/user-attachments/assets/322746be-4225-40b8-b60d-6418a29a6531"/>

A `pinocchio`-based Token program.

## Overview

This repository contains a proof-of-concept of a reimplementation of the SPL Token program, one of the most used programs on Solana, using `pinocchio`. The purpose is to have an implementation that optimizes the compute units, while being fully compatible with the original implementation &mdash; i.e., support the exact same instruction and account layouts as SPL Token, byte for byte.

## Status

| Instruction                | Completed | CU  | CU (SPL Token) |
|----------------------------|-----------|-----|----------------|
| `InitializeMint`           | ✅        | 396 | 2967           |
| `InitializeAccount`        | ✅        | 444 | 4527           |
| `InitializeMultisig`       |           |     |                |
| `Transfer`                 | ✅        | 161 | 4645           |
| `Approve`                  |           |     |                |
| `Revoke`                   |           |     |                |
| `SetAuthority`             |           |     |                |
| `MintTo`                   | ✅        | 160 | 4538           |
| `Burn`                     |           |     |                |
| `CloseAccount`             |           |     |                |
| `FreezeAccount`            |           |     |                |
| `ThawAccount`              |           |     |                |
| `TransferChecked`          |           |     |                |
| `ApproveChecked`           |           |     |                |
| `MintToChecked`            |           |     |                |
| `BurnChecked`              |           |     |                |
| `InitializeAccount2`       |           |     |                |
| `SyncNative`               |           |     |                |
| `InitializeAccount3`       |           |     |                |
| `InitializeMultisig2`      |           |     |                |
| `InitializeMint2`          |           |     |                |
| `GetAccountDataSize`       |           |     |                |
| `InitializeImmutableOwner` |           |     |                |
| `AmountToUiAmount`         |           |     |                |
| `UiAmountToAmount`         |           |     |                |

> Test were run using Solana `v2.1.0`.

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
