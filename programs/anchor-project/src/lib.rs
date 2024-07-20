use anchor_lang::prelude::*;

declare_id!("FTSKdEEWCMPZHgvCpFZLez4WsPqUVAJpaDrHqqec4wXX");

#[program]
pub mod anchor_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from Xolana, my program Pubkey is: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

