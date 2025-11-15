use anchor_lang::prelude::*;

declare_id!("GSKUuDySaKJbVUiHqBa2yLpbVXLL2yZCvcp3oks9jLCL");

#[program]
pub mod stack_underflow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
