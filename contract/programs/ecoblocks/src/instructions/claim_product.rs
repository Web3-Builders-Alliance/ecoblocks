use crate::state::product_info::*;
use crate::state::error_code::*;
use anchor_lang::prelude::*;

pub fn claim_product(ctx: Context<ClaimProduct>) -> Result<()> {
    let product_account = &mut ctx.accounts.product_account;
    let user = &ctx.accounts.user;

    match product_account.claim(user.key()) {
        Ok(_) => Ok(()),
        Err(e) => return Err(e)
    }
}

#[derive(Accounts)]
pub struct ClaimProduct<'info>{
    #[account(mut)]
    pub product_account: Account<'info, ProductInfo>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>
}