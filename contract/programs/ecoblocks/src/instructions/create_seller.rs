use crate::state::seller_info::*;
use crate::state::error_code::*;
use anchor_lang::prelude::*;

pub fn create_seller(ctx: Context<CreateSeller>) -> Result<()> {
    let seller_account = &mut ctx.accounts.seller_account;
    let seller = &ctx.accounts.seller;
    let bump = *ctx.bumps.get("seller_account").ok_or(ErrorsCode::CannotGetBump)?;

    match seller_account.create(seller.key(), bump){
        Ok(_) => Ok(()),
        Err(e) => return Err(e)
    }
}

#[derive(Accounts)]
pub struct CreateSeller<'info>{
    #[account(init, payer = seller, space = SellerInfo::MAXIMUM_SIZE + 8, seeds = [b"seller_account", seller.key.as_ref()], bump)]
    pub seller_account: Account<'info, SellerInfo>,

    #[account(mut)]
    pub seller: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>
}