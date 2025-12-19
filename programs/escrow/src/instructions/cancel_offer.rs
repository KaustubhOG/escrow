use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{close_account, transfer, CloseAccount, Mint, Token, TokenAccount, Transfer},
};

use crate::Offer;

#[derive(Accounts)]
pub struct CancelOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    pub token_mint_a: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = maker,
    )]
    pub maker_token_account_a: Account<'info, TokenAccount>,

    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = token_mint_a,
        seeds = [b"offer", maker.key().as_ref(), offer.id.to_le_bytes().as_ref()],
        bump = offer.bump
    )]
    pub offer: Account<'info, Offer>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer,
    )]
    pub vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn refund_and_close_vault(context: Context<CancelOffer>) -> Result<()> {
    let seeds = &[
        b"offer",
        context.accounts.maker.to_account_info().key.as_ref(),
        &context.accounts.offer.id.to_le_bytes()[..],
        &[context.accounts.offer.bump],
    ];
    let signer_seeds = [&seeds[..]];

    // Transfer tokens from vault back to maker
    let transfer_accounts = Transfer {
        from: context.accounts.vault.to_account_info(),
        to: context.accounts.maker_token_account_a.to_account_info(),
        authority: context.accounts.offer.to_account_info(),
    };

    let cpi_context = CpiContext::new_with_signer(
        context.accounts.token_program.to_account_info(),
        transfer_accounts,
        &signer_seeds,
    );

    transfer(cpi_context, context.accounts.vault.amount)?;

    // Close the vault account and return rent to maker
    let close_accounts = CloseAccount {
        account: context.accounts.vault.to_account_info(),
        destination: context.accounts.maker.to_account_info(),
        authority: context.accounts.offer.to_account_info(),
    };

    let cpi_context = CpiContext::new_with_signer(
        context.accounts.token_program.to_account_info(),
        close_accounts,
        &signer_seeds,
    );

    close_account(cpi_context)
}