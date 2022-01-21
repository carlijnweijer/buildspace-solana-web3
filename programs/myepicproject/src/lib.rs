use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod myepicproject {
  use super::*;

  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    // get a reference to the account
    let base_account = &mut ctx.accounts.base_account;
    // initialize total_goals
    base_account.total_goals = 0;
    Ok(())
  }
}

// attach certain variables to the StartStuffOff context
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// tell Solana what we want to store on this account
#[account]
pub struct BaseAccount {
    pub total_goals: u64,
}


