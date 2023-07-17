use crate::state::product_info::*;
use crate::state::error_code::*;
use anchor_lang::prelude::*;

pub fn recycle_product(ctx: Context<RecycleProduct>) -> Result<()> {
    let product_account = &mut ctx.accounts.product_account;
    let recycler = &ctx.accounts.recycler;

    match product_account.recycle(recycler.key()) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e)
    }
}

#[derive(Accounts)]
pub struct RecycleProduct<'info>{
    #[account(mut)]
    pub product_account: Account<'info, ProductInfo>,

    #[account(mut)]
    pub recycler: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>
}