use anchor_lang::prelude::*;
use solana_program::entrypoint::ProgramResult;

use crate::state::{Idea, Uri};

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct InitializeIdeaParams {
    created_ts: u64,
    uri: String,
}

#[derive(Accounts)]
pub struct InitializeIdea<'info> {
    /// Idea account to initialize
    #[account(init, payer = creator, space = 8 + Idea::LEN)]
    pub idea: Account<'info, Idea>,
    /// Creator of the idea
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<InitializeIdea>, params: InitializeIdeaParams) -> ProgramResult {
    let InitializeIdeaParams { created_ts, uri } = params;

    let idea = &mut ctx.accounts.idea;

    idea.uri = Uri::validate(&uri)?;
    idea.creator = ctx.accounts.creator.key();
    idea.created_ts = created_ts.clone();
    idea.last_updated_ts = created_ts.clone();
    idea.version = 0;

    Ok(())
}
