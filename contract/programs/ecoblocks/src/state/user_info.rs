use anchor_lang::prelude::*;
use crate::state::error_code::*;
use num_derive::*;
use num_traits::*;

#[account]
pub struct UserInfo {
    pub user: Pubkey,
    pub bump: u8
}

impl UserInfo{
    pub const MAXIMUM_SIZE: usize = 32 + 1;

    pub fn create(&mut self, user: Pubkey, bump: u8) -> Result<()> {
        self.user = user;
        self.bump = bump;
        Ok(())
    }
}