use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorsCode {
    #[msg("Coupon Details are too long")]
    CouponDetailsTooLong,

    #[msg("Coupon Details are too short")]
    ProductAlreadyRecycled,

    #[msg("Coupon Details are too short")]
    ProductRemoved,

    #[msg("Cannot get bump")]
    CannotGetBump,

    #[msg("Not in valid time frame")]
    ProductNotInValidTimeWindow,

    #[msg("Recycler not valid")]
    InvalidRecycler
}