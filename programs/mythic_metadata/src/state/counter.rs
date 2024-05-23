use anchor_lang::prelude::*;

use crate::errors::*;

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub bump: u8,
    pub id: u64,
}

impl Counter {
    pub fn size() -> usize {
        8 + // Anchor discriminator
        Counter::INIT_SPACE
    }

    pub fn validate(&self) -> Result<()> {
        if self.id == u64::MAX {
            return err!(MythicMetadataError::CounterIdReachedMax);
        }
        Ok(())
    }
}
