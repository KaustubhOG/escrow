use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>, 

    pub token_mint_a:InterfaceAccount<'info , Mint>,
    pub token_mint_B: InterfaceAccount<'info , Mint>,

    pub maker_tokenaccount_A:InterfaceAccount<'info , Mint>,
    
    pub offer_Pda:
    pub vault_ATA:

    pub system_program:Program<'info , System>,
    pub token_program:Interface<'info , TokenInterface>,
    pub associated_Token_Program:Program<'info , AssociatedToken>,

}
