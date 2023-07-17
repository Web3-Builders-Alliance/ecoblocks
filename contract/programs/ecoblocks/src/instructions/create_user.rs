use crate::state::user_info::*;
use crate::state::error_code::*;
use anchor_lang::prelude::*;

pub fn create_user(ctx: Context<CreateUser>) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;
    let user = &ctx.accounts.user;
    let bump = *ctx.bumps.get("user_account").ok_or(ErrorsCode::CannotGetBump)?;

    match user_account.create(user.key(), bump){
        Ok(_) => Ok(()),
        Err(e) => return Err(e)
    }
}

#[derive(Accounts)]
pub struct CreateUser<'info>{
    #[account(init, payer = user, space = UserInfo::MAXIMUM_SIZE + 8, seeds = [b"user_account", user.key.as_ref()], bump)]
    pub user_account: Account<'info, UserInfo>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>
}