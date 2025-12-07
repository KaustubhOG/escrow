use anchor_lang::prelude::*;
#[account]
#[derive(InitSpace)]
pub struct Offer {
    pub id: u64,
    pub maker_Pub_Key: Pubkey,
    pub Token_mint_A: Pubkey,
    pub Token_mint_B: Pubkey,
    pub Token_B_wanted_amount: u64,
    bump: u8,
}
