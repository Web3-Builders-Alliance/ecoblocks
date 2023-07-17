use anchor_lang::prelude::*;
use crate::state::error_code::*;
use num_derive::*;
use num_traits::*;

#[account]
pub struct UserProductInfo {
    pub user: Pubkey,
    pub product_id: u64,
    pub bump: u8
}

impl UserProductInfo{
    pub const MAXIMUM_SIZE: usize = 32 + 32 + 1;

    pub fn create(&mut self, user: Pubkey, product_id: u64, bump: u8) -> Result<()> {
        self.user = user;
        self.product_id = product_id;
        self.bump = bump;
        Ok(())
    }
}