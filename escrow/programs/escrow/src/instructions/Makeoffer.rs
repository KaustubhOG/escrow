use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(mint::token_program=token_program)]
    pub token_mint_a: InterfaceAccount<'info, Mint>,
    #[account(mint::token_program=token_program)]
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    #[account(
       mut,
       associated_token::mint = token_mint_a,
       associated_token::authority = maker,
       associated_token::token_program = token_program
       )]
    pub maker_token_account_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        mut,
        payer=maker,
        space = 8 + Offer::INIT_SPACE,
        seeds = [b"offer" , maker.key().as_ref() , id.to_le_bytes().as_ref()],
        bump
    )]
    pub offer_pda: Account<'info, Offer>,

    #[account(
    init,
    mut,
    payer = maker,
    associated_token::mint = token_mint_a,
    associated_token::authority = offer_pda,
    associated_token::token_program = token_program
)]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn send_offered_token_vault() -> Result<()> {}
pub fn save_offer() -> Result<()> {}
