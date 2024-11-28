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
| `InitializeMint`           | ✅        | 100            | 2967             |
| `InitializeAccount`        | ✅        | 170            | 4527             |
| `InitializeMultisig`       | ✅        | 190            | 2973             |
| `Transfer`                 | ✅        | 153            | 4645             |
| `Approve`                  | ✅        | 124            | 2904             |
| `Revoke`                   | ✅        |  97            | 2677             |
| `SetAuthority`             | ✅        | 126            | 3167             |
| `MintTo`                   | ✅        | 154            | 4538             |
| `Burn`                     | ✅        | 168            | 4753             |
| `CloseAccount`             | ✅        | 147            | 2916             |
| `FreezeAccount`            | ✅        | 136            | 4265             |
| `ThawAccount`              | ✅        | 136            | 4267             |
| `TransferChecked`          | ✅        | 206            | 6201             |
| `ApproveChecked`           | ✅        | 162            | 4459             |
| `MintToChecked`            | ✅        | 164            | 4546             |
| `BurnChecked`              | ✅        | 170            | 4755             |
| `InitializeAccount2`       | ✅        | 150            | 4388             |
| `SyncNative`               | ✅        |                |                  |
| `InitializeAccount3`       | ✅        | 272            | 4240             |
| `InitializeMultisig2`      | ✅        | 319            | 2826             |
| `InitializeMint2`          | ✅        | 234            | 2827             |
| `GetAccountDataSize`       | ✅        |                |                  |
| `InitializeImmutableOwner` | ✅        |                |                  |
| `AmountToUiAmount`         | ✅        | 483            | 2501             |
| `UiAmountToAmount`         | ✅        | 873            | 3161             |

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
