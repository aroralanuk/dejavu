use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid uri")]
    InvalidURI,
    #[msg("Lockbox can't hold more ideas")]
    ExceededLockboxIdeaLength,
}
