use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod state;

declare_id!("Dvufj2n9dYtimtH8su9ZNHoJ33Yd9a49KAtn5PoJGh2c");

#[program]
pub mod staking {
    use super::*;

    pub fn create_seller(ctx: Context<CreateSeller>) -> Result<()> {
        instructions::create_seller::create_seller(ctx)
    }

    pub fn create_product(ctx: Context<CreateProduct>, id: u64, valid_recyclers: [Pubkey; 3], start_date: i64, end_date: i64, details: String) -> Result<()> {
        instructions::create_product::create_product(ctx, id, valid_recyclers, start_date, end_date, details)
    }

    pub fn create_recycler(ctx: Context<CreateRecycler>) -> Result<()> {
        instructions::create_recycler::create_recycler(ctx)
    }

    pub fn create_user(ctx: Context<CreateUser>) -> Result<()> {
        instructions::create_user::create_user(ctx)
    }

    pub fn claim_product(ctx: Context<ClaimProduct>) -> Result<()> {
        instructions::claim_product::claim_product(ctx)
    }

    pub fn recycle_product(ctx: Context<RecycleProduct>) -> Result<()> {
        instructions::recycle_product::recycle_product(ctx)
    }
}