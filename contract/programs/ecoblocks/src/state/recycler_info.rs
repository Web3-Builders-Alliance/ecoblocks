use anchor_lang::prelude::*;
use crate::state::error_code::*;
use num_derive::*;
use num_traits::*;

#[account]
pub struct RecyclerInfo {
    pub recycler: Pubkey,
    pub bump: u8
}

impl RecyclerInfo{
    pub const MAXIMUM_SIZE: usize = 32 + 1;

    pub fn create(&mut self, recycler: Pubkey, bump: u8) -> Result<()> {
        self.recycler = recycler;
        self.bump = bump;
        Ok(())
    }
}