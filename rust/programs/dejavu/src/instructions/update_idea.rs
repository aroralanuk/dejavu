use anchor_lang::prelude::*;
use solana_program::entrypoint::ProgramResult;

use crate::state::{Idea, Uri};

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct UpdateIdeaParams {
    last_updated_ts: u64,
    uri: String,
}

#[derive(Accounts)]
pub struct UpdateIdea<'info> {
    /// Idea account to initialize
    #[account(mut, constraint = idea.creator == creator.key())]
    pub idea: Account<'info, Idea>,
    /// Creator of the idea
    #[account(mut)]
    pub creator: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateIdea>, params: UpdateIdeaParams) -> ProgramResult {
    let UpdateIdeaParams {
        last_updated_ts,
        uri,
    } = params;

    let idea = &mut ctx.accounts.idea;

    idea.uri = Uri::validate(&uri)?;
    idea.creator = ctx.accounts.creator.key();
    idea.last_updated_ts = last_updated_ts;
    idea.version += 1;

    Ok(())
}
