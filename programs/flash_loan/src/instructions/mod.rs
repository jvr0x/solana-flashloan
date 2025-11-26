pub mod borrow;
pub mod initialize;
pub mod repay;
pub mod set_fee;

pub use initialize::*;
pub use set_fee::*;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::Protocol;
use anchor_lang::solana_program::sysvar::instructions::ID as INSTRUCTIONS_SYSVAR_ID;

#[derive(Accounts)]
pub struct Loan<'info> {
    #[account(mut)]
    pub borrower: Signer<'info>,

    #[account(
        seeds = [b"protocol".as_ref()],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = borrower,
        associated_token::mint = mint,
        associated_token::authority = borrower,
      )]
    pub borrower_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = protocol,
      )]
    pub protocol_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(address = INSTRUCTIONS_SYSVAR_ID)]
    /// CHECK: InstructionsSysvar account
    pub instructions: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

// Account indices for the Loan struct
// These correspond to the order of fields in the Loan struct above
// and are used during instruction introspection to validate account ordering
pub const BORROWER_INDEX: usize = 0;
pub const PROTOCOL_INDEX: usize = 1;
pub const MINT_INDEX: usize = 2;
pub const BORROWER_ATA_INDEX: usize = 3;
pub const PROTOCOL_ATA_INDEX: usize = 4;
pub const INSTRUCTIONS_INDEX: usize = 5;
pub const TOKEN_PROGRAM_INDEX: usize = 6;
pub const ASSOCIATED_TOKEN_PROGRAM_INDEX: usize = 7;
pub const SYSTEM_PROGRAM_INDEX: usize = 8;
