# tokenocchio

A `pinocchio`-based Token program.

This repository contains a proof-of-concept of a reimplementation of the SPL Token program, one of the most used programs on Solana, using `pinocchio`. The purpose is to have an implementation that optimizes the compute units, while being fully compatible with the original implementation &mdash; i.e., support the exact same instruction and account layouts as SPL Token, byte for byte.

## Status

| Instruction                | Completed | CU  | CU (SPL Token) |
|----------------------------|-----------|-----|----------------|
| `InitializeMint`           | [ x ]     | 396 | 2967           |
| `InitializeAccount`        | [ x ]     | 444 | 4527           |
| `InitializeMultisig`       | [ ]       |     |                |
| `Transfer`                 | [ x ]     | 161 | 4645           |
| `Approve`                  | [ ]       |     |                |
| `Revoke`                   | [ ]       |     |                |
| `SetAuthority`             | [ ]       |     |                |
| `MintTo`                   | [ x ]     | 160 | 4538           |
| `Burn`                     | [ ]       |     |                |
| `CloseAccount`             | [ ]       |     |                |
| `FreezeAccount`            | [ ]       |     |                |
| `ThawAccount`              | [ ]       |     |                |
| `TransferChecked`          | [ ]       |     |                |
| `ApproveChecked`           | [ ]       |     |                |
| `MintToChecked`            | [ ]       |     |                |
| `BurnChecked`              | [ ]       |     |                |
| `InitializeAccount2`       | [ ]       |     |                |
| `SyncNative`               | [ ]       |     |                |
| `InitializeAccount3`       | [ ]       |     |                |
| `InitializeMultisig2`      | [ ]       |     |                |
| `InitializeMint2`          | [ ]       |     |                |
| `GetAccountDataSize`       | [ ]       |     |                |
| `InitializeImmutableOwner` | [ ]       |     |                |
| `AmountToUiAmount`         | [ ]       |     |                |
| `UiAmountToAmount`         | [ ]       |     |                |

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
