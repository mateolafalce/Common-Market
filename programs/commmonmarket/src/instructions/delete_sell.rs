use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
}; 
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn delete_sell(
        ctx: Context<DeleteSell>
    ) -> Result<()> {
        require!(ctx.accounts.user.key() == ctx.accounts.offer.pubkey.key(), ErrorCode::PubkeyError);
        let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
        main_account.active_offers -= 1;
        Ok(())
    }

#[derive(Accounts)]
pub struct DeleteSell<'info> {
    #[account(mut, seeds = [b"Main Account"], bump = main_account.bump_original)]
    pub main_account: Account<'info, MainAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds = [offer.seed.to_be_bytes().as_ref()], bump = offer.bump_original, close = user)]
    pub offer: Account<'info, Sell>,
    pub system_program: Program<'info, System>,
}