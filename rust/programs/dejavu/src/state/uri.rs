use anchor_lang::prelude::*;

use crate::error::ErrorCode;

const URI_MAX_LEN: usize = 256;

#[derive(Clone, AnchorDeserialize, AnchorSerialize, Copy)]
pub struct Uri {
    /// length of URI
    pub len: u16,
    /// URI buffer
    pub uri: [u8; URI_MAX_LEN],
}

impl Default for Uri {
    fn default() -> Self {
        Self {
            len: 0,
            uri: [0u8; URI_MAX_LEN],
        }
    }
}

impl Uri {
    pub fn validate(uri: &str) -> Result<Uri> {
        let len = uri.len();
        if len > URI_MAX_LEN {
            return Err(error!(ErrorCode::InvalidURI));
        }

        let mut bytes = [0; URI_MAX_LEN];
        bytes[..len].copy_from_slice(uri.as_bytes());

        Ok(Uri {
            len: len as u16,
            uri: bytes,
        })
    }

    pub const LEN: usize = 2 + URI_MAX_LEN;
}
