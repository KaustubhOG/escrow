use anchor_lang::prelude::*;

declare_id!("GLNf3WrkLgAJT8vumGA6SGv5oPCnqyVrRX3AY3th3Nby");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
