pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("22222222222222222222222222222222222222222222");

#[program]
pub mod flash_loan {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, authority: Pubkey, initial_fee: u64) -> Result<()> {
        initialize::handler(ctx, authority, initial_fee)
    }

    pub fn set_fee(ctx: Context<SetFee>, fee: u64) -> Result<()> {
        set_fee::handler(ctx, fee)
    }

    pub fn borrow(ctx: Context<Loan>, borrow_amount: u64) -> Result<()> {
        borrow::handler(ctx, borrow_amount)
    }

    pub fn repay(ctx: Context<Loan>) -> Result<()> {
        repay::handler(ctx)
    }
}
