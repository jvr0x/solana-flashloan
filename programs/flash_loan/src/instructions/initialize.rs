use anchor_lang::prelude::*;

use crate::{error::ErrorCode, Protocol};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = Protocol::INIT_SPACE,
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,

    pub system_program: Program<'info, System>,
}

/// Initializes the protocol configuration PDA
/// Sets the initial authority and flash loan fee
pub fn handler(ctx: Context<Initialize>, authority: Pubkey, initial_fee: u64) -> Result<()> {
    require!(initial_fee > 0u64, ErrorCode::InvalidFee);
    require!(initial_fee <= 1_000u64, ErrorCode::InvalidFee);

    let protocol = &mut ctx.accounts.protocol;
    protocol.authority = authority;
    protocol.fee = initial_fee;

    msg!(
        "Protocol initialized with authority: {:?}, fee: {} bps",
        authority,
        initial_fee
    );

    Ok(())
}
