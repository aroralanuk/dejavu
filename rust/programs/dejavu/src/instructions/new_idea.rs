use anchor_lang::prelude::*;
use solana_program::entrypoint::ProgramResult;

use crate::error::ErrorCode;
use crate::state::{Idea, IdeaWrapper, Lockbox, Uri, MAX_IDEA_LEN};

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct NewIdeaParams {
    uri: String,
}

#[derive(Accounts)]
pub struct NewIdea<'info> {
    /// Idea account to initialize
    #[account(
        init,
        payer = creator,
        seeds = [b"idea", lockbox.key().as_ref(), &lockbox.load()?.num_ideas.to_le_bytes()],
        bump,
        space = 8 + Idea::LEN
    )]
    pub idea: Account<'info, Idea>,
    /// Creator of the idea
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut, constraint = creator.key() == lockbox.load()?.creator)]
    pub lockbox: AccountLoader<'info, Lockbox>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl NewIdea<'_> {
    pub fn can_initialize(&self) -> Result<()> {
        if self.lockbox.load()?.num_ideas + 1 >= MAX_IDEA_LEN {
            return Err(error!(ErrorCode::ExceededLockboxIdeaLength));
        }
        Ok(())
    }
}

pub fn handler(ctx: Context<NewIdea>, params: NewIdeaParams) -> ProgramResult {
    let NewIdeaParams { uri } = params;

    let created_ts = Clock::get()?.unix_timestamp as u64;

    let idea = &mut ctx.accounts.idea;
    let lockbox = &mut ctx.accounts.lockbox.load_mut()?;

    idea.uri = Uri::validate(&uri)?;
    idea.creator = ctx.accounts.creator.key();
    idea.created_ts = created_ts;
    idea.last_updated_ts = created_ts;
    idea.version = 0;

    lockbox.last_updated_ts = created_ts;
    lockbox.num_ideas += 1;

    let idx = lockbox.num_ideas;
    lockbox.ideas[idx] = IdeaWrapper { idea: idea.key() };

    Ok(())
}
