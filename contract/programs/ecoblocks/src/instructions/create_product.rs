use crate::state::product_info::*;
use crate::state::error_code::*;
use anchor_lang::prelude::*;

pub fn create_product(ctx: Context<CreateProduct>, id: u64, valid_recyclers: [Pubkey; 3], start_date: i64, end_date: i64, coupon_details: String) -> Result<()> {
    let product_account = &mut ctx.accounts.product_account;
    let seller = &ctx.accounts.seller;
    let bump = *ctx.bumps.get("product_account").ok_or(ErrorsCode::CannotGetBump)?;

    match product_account.create(id, seller.key(), valid_recyclers, start_date, end_date, coupon_details, bump){
        Ok(_) => Ok(()),
        Err(e) => return Err(e)
    }
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreateProduct<'info>{
    #[account(init, payer = seller, space = ProductInfo::MAXIMUM_SIZE + 8, seeds = [b"product_account", id.to_le_bytes().as_ref(), seller.key.as_ref()], bump)]
    pub product_account: Account<'info, ProductInfo>,

    #[account(mut)]
    pub seller: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>
}