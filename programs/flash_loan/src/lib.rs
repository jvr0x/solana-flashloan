pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("G5fiBBkNj5BtaDjo9ZV3nCUtd7mDSc5KhhPDweVzFRW6");

#[program]
pub mod flash_loan {
    use super::*;

    pub fn borrow(ctx: Context<Loan>) -> Result<()> {
        borrow::handler(ctx)
    }
    
    pub fn repay(ctx: Context<Loan>) -> Result<()> {
        repay::handler(ctx)
    }
}
