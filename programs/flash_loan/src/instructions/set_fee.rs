use anchor_lang::prelude::*;

use crate::{error::ErrorCode, Protocol};

#[derive(Accounts)]
pub struct SetFee<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"protocol"],
        bump,
        has_one = authority,
    )]
    pub protocol: Account<'info, Protocol>,
}

/// Updates the protocol flash loan fee
/// Fee must be between 0-1000 basis points (0-10%)
pub fn handler(ctx: Context<SetFee>, fee: u64) -> Result<()> {
    require!(fee > 0u64, ErrorCode::InvalidFee);
    require!(fee <= 1_000u64, ErrorCode::InvalidFee);

    ctx.accounts.protocol.fee = fee;

    msg!("Protocol fee updated to {} bps", fee);

    Ok(())
}
