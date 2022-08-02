use anchor_lang::prelude::*;
use solana_program::entrypoint::ProgramResult;

use crate::state::{Idea, Lockbox, Uri};

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct UpdateIdeaParams {
    uri: String,
}

#[derive(Accounts)]
pub struct UpdateIdea<'info> {
    /// Idea account to initialize
    #[account(mut, constraint = idea.creator == creator.key())]
    pub idea: Account<'info, Idea>,
    #[account(mut, constraint = lockbox.load()?.creator == creator.key())]
    pub lockbox: AccountLoader<'info, Lockbox>,
    /// Creator of the idea
    #[account(mut)]
    pub creator: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateIdea>, params: UpdateIdeaParams) -> ProgramResult {
    let UpdateIdeaParams { uri } = params;

    let idea = &mut ctx.accounts.idea;
    let lockbox = &mut ctx.accounts.lockbox.load_mut()?;
    let current_ts = Clock::get()?.unix_timestamp as u64;

    idea.uri = Uri::validate(&uri)?;
    idea.creator = ctx.accounts.creator.key();
    idea.last_updated_ts = current_ts;
    idea.version += 1;

    lockbox.last_updated_ts = current_ts;

    Ok(())
}
