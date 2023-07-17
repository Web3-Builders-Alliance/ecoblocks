use crate::state::product_info::*;
use crate::state::user_product_info::*;
use crate::state::error_code::*;
use anchor_lang::prelude::*;

pub fn claim_product(ctx: Context<ClaimProduct>) -> Result<()> {
    let product_account = &mut ctx.accounts.product_account;
    let user = &ctx.accounts.user;

    match product_account.claim(user.key()) {
        Ok(_) => {
            let user_product_account = &mut ctx.accounts.user_product_account;
            let bump = *ctx.bumps.get("user_product_account").ok_or(ErrorsCode::CannotGetBump)?;
            match user_product_account.create(user.key(), product_account.id, bump) {
                Ok(_) => Ok(()),
                Err(e) => return Err(e)
            }
        },
        Err(e) => return Err(e)
    }
}

#[derive(Accounts)]
pub struct ClaimProduct<'info>{
    #[account(mut)]
    pub product_account: Account<'info, ProductInfo>,

    #[account(init, payer = user, space = UserProductInfo::MAXIMUM_SIZE + 8, seeds = [b"user_product_account", product_account.id.to_le_bytes().as_ref(), user.key.as_ref()], bump)]
    pub user_product_account: Account<'info, UserProductInfo>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>
}