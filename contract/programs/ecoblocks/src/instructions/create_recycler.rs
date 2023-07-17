use crate::state::recycler_info::*;
use crate::state::error_code::*;
use anchor_lang::prelude::*;

pub fn create_recycler(ctx: Context<CreateRecycler>) -> Result<()> {
    let recycler_account = &mut ctx.accounts.recycler_account;
    let recycler = &ctx.accounts.recycler;
    let bump = *ctx.bumps.get("recycler_account").ok_or(ErrorsCode::CannotGetBump)?;

    match recycler_account.create(recycler.key(), bump){
        Ok(_) => Ok(()),
        Err(e) => return Err(e)
    }
}

#[derive(Accounts)]
pub struct CreateRecycler<'info>{
    #[account(init, payer = recycler, space = RecyclerInfo::MAXIMUM_SIZE + 8, seeds = [b"recycler_account", recycler.key.as_ref()], bump)]
    pub recycler_account: Account<'info, RecyclerInfo>,

    #[account(mut)]
    pub recycler: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>
}