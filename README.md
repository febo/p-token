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

> [!WARNING]
> The program is not yet fully-optimized. There are still opportunities to improve the compute units consumption.

## Overview

This repository contains a **proof-of-concept** of a reimplementation of the SPL Token program, one of the most used programs on Solana, using [`pinocchio`](https://github.com/febo/pinocchio). The purpose is to have an implementation that optimizes the compute units, while being fully compatible with the original implementation &mdash; i.e., support the exact same instruction and account layouts as SPL Token, byte for byte.

## Status

| Instruction                | Completed | CU (`p-token`) | CU (`spl-token`) |
|----------------------------|-----------|----------------|------------------|
| `InitializeMint`           | ✅        | 361            | 2967             |
| `InitializeAccount`        | ✅        | 430            | 4527             |
| `InitializeMultisig`       | ✅        | 454            | 2973             |
| `Transfer`                 | ✅        | 159            | 4645             |
| `Approve`                  | ✅        | 144            | 2904             |
| `Revoke`                   | ✅        | 110            | 2677             |
| `SetAuthority`             | ✅        | 153            | 3167             |
| `MintTo`                   | ✅        | 160            | 4538             |
| `Burn`                     | ✅        | 180            | 4753             |
| `CloseAccount`             | ✅        | 158            | 2916             |
| `FreezeAccount`            | ✅        | 149            | 4265             |
| `ThawAccount`              | ✅        | 150            | 4267             |
| `TransferChecked`          | ✅        | 224            | 6201             |
| `ApproveChecked`           | ✅        | 185            | 4459             |
| `MintToChecked`            | ✅        | 191            | 4546             |
| `BurnChecked`              | ✅        | 182            | 4755             |
| `InitializeAccount2`       | ✅        | 414            | 4388             |
| `SyncNative`               | ✅        |                |                  |
| `InitializeAccount3`       | ✅        | 518            | 4240             |
| `InitializeMultisig2`      | ✅        | 584            | 2826             |
| `InitializeMint2`          | ✅        | 495            | 2827             |
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
