use anchor_lang::prelude::*;

use crate::state::Uri;

#[account]
#[derive(Copy, Default)]
pub struct Idea {
    /// Owner of the idea
    pub creator: Pubkey,
    /// Time of origin of the idea
    pub created_ts: u64,
    /// Last updated version of the idea
    pub last_updated_ts: u64,
    /// Number of iterations on the idea
    pub version: u32,
    /// URI to the idea
    pub uri: Uri,
}

impl Idea {
    pub const LEN: usize = 32 + 2 * 8 + 4 + Uri::LEN;
}
