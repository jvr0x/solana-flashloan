# Flash Loan Program

A Solana flash loan protocol built with the Anchor framework. This program enables users to borrow tokens without collateral, provided the loan is repaid within the same transaction.

## Overview

Flash loans are uncollateralized loans that must be borrowed and repaid atomically within a single transaction. If the repayment fails, the entire transaction reverts, ensuring the protocol never loses funds.

This implementation uses **instruction introspection** to verify that a `repay` instruction exists in the same transaction before releasing borrowed funds.

## Features

- Uncollateralized instant loans
- Configurable protocol fee (in basis points)
- Atomic borrow/repay enforcement via instruction introspection
- Support for any SPL token (Token Program and Token-2022)
- PDA-based protocol authority for secure fund management

## Architecture

### Accounts

| Account | Type | Description |
|---------|------|-------------|
| `Protocol` | PDA | Stores protocol configuration (authority, fee). Seeds: `[b"protocol"]` |
| `protocol_ata` | ATA | Protocol's token account holding loanable liquidity |
| `borrower_ata` | ATA | Borrower's token account for receiving/repaying loans |

### Instructions

| Instruction | Description |
|-------------|-------------|
| `initialize` | Creates the Protocol PDA and sets initial authority and fee |
| `set_fee` | Updates the protocol fee (authority only) |
| `borrow` | Borrows tokens from the protocol (requires `repay` in same tx) |
| `repay` | Repays borrowed tokens plus fee |

## How It Works

1. **Borrow Phase**: When `borrow` is called, the program:
   - Validates the borrow amount
   - Uses instruction introspection to verify a `repay` instruction exists later in the transaction
   - Transfers tokens from `protocol_ata` to `borrower_ata`

2. **Usage Phase**: The borrower can use the funds for any purpose (arbitrage, liquidations, etc.) in subsequent instructions within the same transaction.

3. **Repay Phase**: When `repay` is called, the program:
   - Reads the borrow amount from the `borrow` instruction data
   - Calculates the fee based on protocol configuration
   - Transfers `amount + fee` from `borrower_ata` back to `protocol_ata`

If any step fails, the entire transaction reverts.

## Fee Structure

Fees are configured in **basis points** (1 basis point = 0.01%):

| Fee (bps) | Percentage |
|-----------|------------|
| 5 | 0.05% |
| 50 | 0.5% |
| 100 | 1% |
| 1000 | 10% (maximum) |

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor](https://www.anchor-lang.com/docs/installation)
- [Node.js](https://nodejs.org/) and Yarn

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd flash_loan

# Install dependencies
yarn install

# Build the program
anchor build
```

### Testing

```bash
# Start local validator
solana-test-validator

# Run tests
anchor test
```

### Deployment

```bash
# Deploy to localnet
anchor deploy

# Deploy to devnet
anchor deploy --provider.cluster devnet
```

## Usage Example

A typical flash loan transaction contains:

```
Transaction:
  1. borrow(amount: 1000000000)    // Borrow 10 tokens (8 decimals)
  2. ... your operations ...       // Arbitrage, liquidation, etc.
  3. repay()                       // Repay loan + fee
```

### Client Example (TypeScript)

```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FlashLoan } from "../target/types/flash_loan";

// Initialize protocol (one-time setup)
await program.methods
  .initialize(authority.publicKey, new anchor.BN(50)) // 0.5% fee
  .accounts({
    payer: payer.publicKey,
    protocol: protocolPda,
    systemProgram: SystemProgram.programId,
  })
  .signers([payer])
  .rpc();

// Flash loan transaction
const tx = new Transaction();

// Add borrow instruction
tx.add(
  await program.methods
    .borrow(new anchor.BN(1_000_000_000))
    .accounts({
      borrower: borrower.publicKey,
      protocol: protocolPda,
      mint: tokenMint,
      borrowerAta: borrowerAta,
      protocolAta: protocolAta,
      instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    })
    .instruction()
);

// Add your operations here...

// Add repay instruction
tx.add(
  await program.methods
    .repay()
    .accounts({
      borrower: borrower.publicKey,
      protocol: protocolPda,
      mint: tokenMint,
      borrowerAta: borrowerAta,
      protocolAta: protocolAta,
      instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    })
    .instruction()
);

await sendAndConfirmTransaction(connection, tx, [borrower]);
```

## Error Codes

| Code | Name | Description |
|------|------|-------------|
| `InvalidIx` | Invalid instruction | Instruction discriminator mismatch |
| `InvalidInstructionIndex` | Invalid instruction index | Unexpected instruction position |
| `InvalidInstructionOrder` | Invalid instruction order | Borrow/repay not in correct order |
| `InvalidAmount` | Invalid amount | Borrow amount must be > 0 |
| `NotEnoughFunds` | Not enough funds | Insufficient protocol liquidity |
| `InvalidProgram` | Invalid program | Instruction from wrong program |
| `InvalidBorrowerAta` | Invalid borrower ATA | Token account mismatch |
| `InvalidProtocolAta` | Invalid protocol ATA | Token account mismatch |
| `MissingRepayIx` | Missing repay instruction | No repay found in transaction |
| `MissingBorrowIx` | Missing borrow instruction | No borrow found in transaction |
| `Overflow` | Overflow | Arithmetic overflow in fee calculation |
| `InvalidFee` | Invalid fee | Fee exceeds maximum (1000 bps) |

## Security Considerations

1. **Atomic Execution**: The instruction introspection ensures borrow and repay happen atomically
2. **PDA Authority**: Only the program can sign for token transfers from the protocol
3. **Fee Validation**: Maximum fee capped at 10% to prevent configuration errors
4. **Account Validation**: Strict verification of token accounts in repay matches borrow

## License

MIT
