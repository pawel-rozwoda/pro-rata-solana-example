use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};


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
        //pub v: Vec<Auth>,
        pub authority: Pubkey,
        pub count: i64,
        pub share: i64,
        pub deposition: i64,
        pub claims: [bool;5],
        pub addresses: [i8; 5],
        pub claim_available: bool,
        pub share_to_claim: i64,
    }

    impl Counter {
        pub fn new(ctx: Context<Auth>) -> Result<Self> {
            Ok(Self {
                claim_available: false,
                claims: [false; 5],
                addresses: [0,1,2,3,4],
                authority: *ctx.accounts.authority.key,
                count: 0,
                share: 0,
                deposition: 100,
                share_to_claim: 0,

            })
        }


        pub fn deposit(&mut self, ctx: Context<Auth>, data: u16) -> Result<()> {
            if &self.authority != ctx.accounts.authority.key {
                return Err(ErrorCode::Unauthorized.into());
            }
            self.claims[data as usize] = true;
            self.count += 1;
            self.share += self.deposition;
            if self.count == 5 {
                self.claim_available = true;
                self.share += self.deposition;
                self.share_to_claim = self.share / self.count;
            }
            Ok(())
        }


        
    }

}



#[derive(Accounts)]
pub struct Auth<'info> {
    #[account(signer)]
    authority: AccountInfo<'info>,
    //addr: i32,
}
// #endregion code

#[error]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
}
