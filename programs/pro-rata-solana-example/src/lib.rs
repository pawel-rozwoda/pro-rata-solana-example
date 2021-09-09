use anchor_lang::prelude::*;

#[program]
pub mod pro_rata_solana_example {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
