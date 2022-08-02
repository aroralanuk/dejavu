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

    pub fn initialize_idea(
        ctx: Context<InitializeIdea>,
        params: InitializeIdeaParams,
    ) -> ProgramResult {
        instructions::initialize_idea::handler(ctx, params)
    }
}
