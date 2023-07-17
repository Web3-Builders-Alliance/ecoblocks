use anchor_lang::{prelude::*, Bump};
use crate::state::error_code::*;
use num_derive::*;
use num_traits::*;

#[derive(Clone, Copy, Debug, PartialEq, AnchorSerialize, AnchorDeserialize, FromPrimitive)]
pub enum ProductStatus {
    Listed,
    Removed,
    Claimed,
    Recycled,
}

#[account]
pub struct ProductInfo {
    pub id: u64,
    pub seller: Pubkey,  // The brand pubkey
    pub valid_recyclers: [Pubkey; 3],  // The recyclers pubkeys
    pub owner: Option<Pubkey>,  // The user pubkey
    pub recycler: Option<Pubkey>,  // The recycler pubkey
    pub status: ProductStatus,
    pub start_date: i64,
    pub end_date: i64,
    pub coupon_details: String,
    pub bump: u8
}

impl ProductInfo{
    pub const MAXIMUM_SIZE: usize = 8 + 32 + 32 * 3 + 32 + 32 + 3 + 8 + 8 + 200 + 1;

    pub fn create(&mut self, id: u64, seller: Pubkey, valid_recyclers: [Pubkey; 3], start_date: i64, end_date: i64, coupon_details: String, bump: u8) -> Result<()> {
        if coupon_details.len() > 200 {
            return Err(ErrorsCode::CouponDetailsTooLong.into());
        }
        self.id = id;
        self.seller = seller;
        self.valid_recyclers = valid_recyclers;
        self.owner = None;
        self.recycler = None;
        self.status = ProductStatus::Listed;
        self.start_date = start_date;
        self.end_date = end_date;
        self.coupon_details = coupon_details;
        self.bump = bump;
        Ok(())
    }

    pub fn get_status(&self) -> ProductStatus {
        self.status
    }

    pub fn claim(&mut self, user_key: Pubkey) -> Result<()> {
        if self.status == ProductStatus::Recycled {
            return Err(ErrorsCode::ProductAlreadyRecycled.into());
        }
        if self.status == ProductStatus::Removed {
            return Err(ErrorsCode::ProductRemoved.into());
        }
        self.status = ProductStatus::Claimed;
        self.owner = Some(user_key);
        Ok(())
    }

    pub fn recycle(&mut self, recycler: Pubkey) -> Result<()> {
        if self.status == ProductStatus::Recycled {
            return Err(ErrorsCode::ProductAlreadyRecycled.into());
        }
        if self.status == ProductStatus::Removed {
            return Err(ErrorsCode::ProductRemoved.into());
        }
        self.status = ProductStatus::Recycled;
        self.recycler = Some(recycler);
        Ok(())
    }

    pub fn remove(&mut self) -> Result<()> {
        if self.status == ProductStatus::Recycled {
            return Err(ErrorsCode::ProductAlreadyRecycled.into());
        }
        self.status = ProductStatus::Removed;
        Ok(())
    }
}
