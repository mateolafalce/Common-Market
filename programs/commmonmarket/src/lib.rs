use anchor_lang::prelude::*;
use instructions::*;
use crate::errors::ErrorCode;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("7N6EPRN8bWZBgnDhxShPbfDqrGXUjCDRvqiGTpaUtc33");

#[program]
pub mod commonmarket {
    use super::*;
    pub fn main_account(ctx: Context<Create>) -> Result<()> {
        instructions::main_account::main_account(ctx)
    }
    pub fn sell(
        ctx: Context<SellStruct>,
        product: String,
        description: String,
        supply: u64,
        price: u64,
        ipfs_url: String
    ) -> Result<()> {
        instructions::sell::sell(
            ctx,
            product,
            description,
            supply,
            price,
            ipfs_url
        )
    }
    pub fn exchange(
        ctx: Context<Exchange>,
        supply: u64
    ) -> Result<()> {
        instructions::exchange::exchange(
            ctx,
            supply
        )
    }
    pub fn update_supply(
        ctx: Context<UpdateSupply>,
        supply: u64
    ) -> Result<()> {
        instructions::update_supply::update_supply(
            ctx,
            supply
        )
    }
    pub fn delete_sell(ctx: Context<DeleteSell>) -> Result<()> {
        instructions::delete_sell::delete_sell(ctx)
    }
}