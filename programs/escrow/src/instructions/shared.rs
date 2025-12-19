use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

pub fn transfer_tokens<'info>(
    from: &Account<'info, TokenAccount>,
    to: &Account<'info, TokenAccount>,
    amount: &u64,
    mint: &Account<'info, Mint>,
    authority: &Signer<'info>,
    token_program: &Program<'info, Token>,
) -> Result<()> {
    let transfer_accounts = Transfer {
        from: from.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
    };

    let cpi_context = CpiContext::new(token_program.to_account_info(), transfer_accounts);

    transfer(cpi_context, *amount)
}