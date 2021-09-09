use anchor_lang::prelude::*;

//#[program]
//pub mod pro_rata_solana_example {
    //use super::*;
    //pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        //Ok(())
    //}
//}

//#[derive(Accounts)]
//pub struct Initialize {}

#[program]
pub mod pro_rata_solana_example {
    use super::*;

    #[state]
    pub struct Counter {
        pub authority: Pubkey,
        pub count: u64,
        pub share: u64,
        pub deposition: u64,
    }

    impl Counter {
        pub fn new(ctx: Context<Auth>) -> Result<Self> {
            Ok(Self {
                authority: *ctx.accounts.authority.key,
                count: 0,
                share: 0,
                deposition: 10,
            })
        }


        pub fn deposit(&mut self, ctx: Context<Auth>) -> Result<()> {
            if &self.authority != ctx.accounts.authority.key {
                return Err(ErrorCode::Unauthorized.into());
            }
            self.count += 1;
            self.share += self.deposition;
            Ok(())
        }

        
    }

}



#[derive(Accounts)]
pub struct Auth<'info> {
    #[account(signer)]
    authority: AccountInfo<'info>,
}
// #endregion code

#[error]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
}
