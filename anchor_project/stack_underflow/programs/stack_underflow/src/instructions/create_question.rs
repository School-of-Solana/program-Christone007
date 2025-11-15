use anchor_lang::prelude::*;

use crate::errors::StackError;
use crate::states::*;

pub fn create_question(ctx: Context<CreateQuestion>, topic: String, body: String) -> Result<()> {
    let question = &mut ctx.accounts.question;
    let creator = &mut ctx.accounts.question_authority;
    let question_topic = topic;
    let question_body = body;

    require!(
        question_topic.len() <= QUESTION_TOPIC_LENGTH,
        StackError::QuestionTopicTooLong
    );

    require!(
        question_body.len() <= QUESTION_BODY_LENGTH,
        StackError::QuestionBodyTooLong
    );

    question.answer_count = 0;
    question.question_creator = creator.to_account_info().key();
    question.question_topic = question_topic;
    question.question_body = question_body;

    Ok(())
}

#[derive(Accounts)]
#[instruction(topic: String)]
pub struct CreateQuestion<'info> {
    #[account(mut)]
    pub question_authority: Signer<'info>,
    #[account(
        init,
        payer=question_authority,
        space=8 + Question::INIT_SPACE,
        seeds=[QUESTION_SEED.as_bytes(), topic.as_bytes()], // question_authority is not part of the seed, hence pubKey not required to generate the PDA. Anyone can generate the question's pda deterministically without knowing who created it
        bump,
    )]
    pub question: Account<'info, Question>,
    pub system_program: Program<'info, System>,
}
