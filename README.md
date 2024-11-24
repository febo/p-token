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
| `InitializeMint`           | ✅        | 343            | 2967             |
| `InitializeAccount`        | ✅        | 416            | 4527             |
| `InitializeMultisig`       | ✅        | 499            | 2973             |
| `Transfer`                 | ✅        | 140            | 4645             |
| `Approve`                  | ✅        | 133            | 2904             |
| `Revoke`                   | ✅        | 106            | 2677             |
| `SetAuthority`             | ✅        | 142            | 3167             |
| `MintTo`                   | ✅        | 143            | 4538             |
| `Burn`                     | ✅        | 175            | 4753             |
| `CloseAccount`             | ✅        | 147            | 2916             |
| `FreezeAccount`            | ✅        | 141            | 4265             |
| `ThawAccount`              | ✅        | 142            | 4267             |
| `TransferChecked`          | ✅        | 211            | 6201             |
| `ApproveChecked`           | ✅        | 169            | 4459             |
| `MintToChecked`            | ✅        | 178            | 4546             |
| `BurnChecked`              | ✅        | 181            | 4755             |
| `InitializeAccount2`       | ✅        | 399            | 4388             |
| `SyncNative`               | ✅        |                |                  |
| `InitializeAccount3`       | ✅        | 508            | 4240             |
| `InitializeMultisig2`      | ✅        | 579            | 2826             |
| `InitializeMint2`          | ✅        | 477            | 2827             |
| `GetAccountDataSize`       | ✅        |                |                  |
| `InitializeImmutableOwner` | ✅        |                |                  |
| `AmountToUiAmount`         | ✅        |                |                  |
| `UiAmountToAmount`         | ✅        |                |                  |

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
