use anchor_lang::prelude::*;

use crate::errors::StackError;
use crate::states::*;

pub fn create_question(ctx: CreateQuestion, topic: String, body: String) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct CreateQuestion<'info> {
    pub question_authority: Signer<'info>,
    pub question: Account<'info, Question>,
}
