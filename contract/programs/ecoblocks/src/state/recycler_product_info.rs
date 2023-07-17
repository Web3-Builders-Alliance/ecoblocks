use anchor_lang::prelude::*;
use crate::state::error_code::*;
use num_derive::*;
use num_traits::*;

#[account]
pub struct RecyclerProductInfo {
    pub recycler: Pubkey,
    pub product_id: u64,
    pub bump: u8
}

impl RecyclerProductInfo{
    pub const MAXIMUM_SIZE: usize = 32 + 32 + 1;

    pub fn create(&mut self, recycler: Pubkey, product_id: u64, bump: u8) -> Result<()> {
        self.recycler = recycler;
        self.product_id = product_id;
        self.bump = bump;
        Ok(())
    }
}