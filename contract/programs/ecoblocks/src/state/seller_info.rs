use anchor_lang::prelude::*;
use crate::state::error_code::*;
use num_derive::*;
use num_traits::*;

#[account]
pub struct SellerInfo {
    pub seller: Pubkey,
    pub bump: u8
}

impl SellerInfo{
    pub const MAXIMUM_SIZE: usize = 32 + 1;

    pub fn create(&mut self, seller: Pubkey, bump: u8) -> Result<()> {
        self.seller = seller;
        self.bump = bump;
        Ok(())
    }
}