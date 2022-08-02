use anchor_lang::prelude::*;
use solana_program::entrypoint::ProgramResult;

pub mod error;
pub mod instructions;
pub mod state;

use crate::instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod dejavu {
    use super::*;

    pub fn new_idea(ctx: Context<NewIdea>, params: NewIdeaParams) -> ProgramResult {
        instructions::new_idea::handler(ctx, params)
    }

    pub fn update_idea(ctx: Context<UpdateIdea>, params: UpdateIdeaParams) -> ProgramResult {
        instructions::update_idea::handler(ctx, params)
    }

    pub fn initialize_lockbox(ctx: Context<InitializeLockbox>) -> ProgramResult {
        instructions::initialize_lockbox::handler(ctx)
    }
}
