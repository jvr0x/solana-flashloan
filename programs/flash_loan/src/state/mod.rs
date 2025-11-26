use anchor_lang::prelude::*;

/// Protocol configuration account stored as a PDA
/// Stores the protocol authority and flash loan fee in basis points
#[account]
#[derive(InitSpace)]
pub struct Protocol {
    /// Authority that can update protocol settings
    pub authority: Pubkey,
    /// Flash loan fee in basis points (1/10000)
    /// Example: 50 = 0.5%, 100 = 1%
    pub fee: u64,
}
