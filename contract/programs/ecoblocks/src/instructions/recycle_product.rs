use crate::state::product_info::*;
use crate::state::recycler_product_info::*;
use crate::state::error_code::*;
use anchor_lang::prelude::*;

pub fn recycle_product(ctx: Context<RecycleProduct>) -> Result<()> {
    let product_account = &mut ctx.accounts.product_account;
    let recycler = &ctx.accounts.recycler;

    match product_account.recycle(recycler.key()) {
        Ok(_) => {
            let recycler_product_account = &mut ctx.accounts.recycler_product_account;
            let bump = *ctx.bumps.get("recycler_product_account").ok_or(ErrorsCode::CannotGetBump)?;
            match recycler_product_account.create(recycler.key(), product_account.id, bump) {
                Ok(_) => Ok(()),
                Err(e) => return Err(e)
            }
        },
        Err(e) => return Err(e)
    }
}

#[derive(Accounts)]
pub struct RecycleProduct<'info>{
    #[account(mut)]
    pub product_account: Account<'info, ProductInfo>,

    #[account(init, payer = recycler, space = RecyclerProductInfo::MAXIMUM_SIZE + 8, seeds = [b"recycler_product_account", product_account.id.to_le_bytes().as_ref(), recycler.key.as_ref()], bump)]
    pub recycler_product_account: Account<'info, RecyclerProductInfo>,

    #[account(mut)]
    pub recycler: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>
}