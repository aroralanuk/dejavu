use anchor_lang::prelude::*;
use solana_program::entrypoint::ProgramResult;

use crate::state::{IdeaWrapper, Lockbox, MAX_IDEA_LEN};

#[derive(Accounts)]
pub struct InitializeLockbox<'info> {
    /// Lockbox account to initialize
    #[account(
        init,
        payer = creator,
        seeds = [b"lockbox", creator.key().as_ref()],
        bump,
        space = 8 + Lockbox::LEN
    )]
    pub lockbox: AccountLoader<'info, Lockbox>,
    /// Creator of the idea
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<InitializeLockbox>) -> ProgramResult {
    let created_ts = Clock::get()?.unix_timestamp as u64;

    let lockbox = &mut ctx.accounts.lockbox.load_init()?;

    lockbox.creator = ctx.accounts.creator.key();
    lockbox.created_ts = created_ts;
    lockbox.last_updated_ts = created_ts;
    lockbox.num_ideas = 0;
    lockbox.ideas = [IdeaWrapper {
        idea: Pubkey::default(),
    }; MAX_IDEA_LEN];

    Ok(())
}
