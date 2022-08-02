use anchor_lang::prelude::*;

pub const MAX_IDEA_LEN: usize = 1024;

#[account(zero_copy)]
pub struct Lockbox {
    pub creator: Pubkey,
    pub created_ts: u64,
    pub last_updated_ts: u64,
    pub num_ideas: u64,
    pub ideas: [IdeaWrapper; MAX_IDEA_LEN],
}

#[zero_copy]
pub struct IdeaWrapper {
    pub idea: Pubkey,
}

impl Lockbox {
    pub const LEN: usize = 32 + 3 * 8 + 32 * MAX_IDEA_LEN;
}
