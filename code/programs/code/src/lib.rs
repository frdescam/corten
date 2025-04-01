use anchor_lang::prelude::*;

declare_id!("H9P4R2DfwRFE8ch3d6z8sATgH3deshrSgoND1zkK8AK3");

#[program]
pub mod code {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
