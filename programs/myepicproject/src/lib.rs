use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    base_account.total_goals = 0;
    Ok(())
  }

  pub fn add_goal(ctx: Context<AddGoal>, goal_goal: String, goal_deadline: String) -> ProgramResult {
      let base_account = &mut ctx.accounts.base_account;
      let user = &mut ctx.accounts.user;

      //build the struct
      let item = ItemStruct {
          goal_goal: goal_goal.to_string(),
          goal_deadline: goal_deadline.to_string(),
          user_address: *user.to_account_info().key,
      };

      //add it to the goal_list vector
      base_account.goal_list.push(item);
      base_account.total_goals += 1;
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


// add the signer who calls the AddGoal method to the struct so that we can save it
#[derive(Accounts)]
pub struct AddGoal<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

//tells Anchor how to serialize /deserialize the struct
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub goal_goal: String,
    pub goal_deadline: String,
    pub user_address: Pubkey,
}

// tell Solana what we want to store on this account
#[account]
pub struct BaseAccount {
    pub total_goals: u64,
    // attach a vector of type itemstruct to the account
    pub goal_list: Vec<ItemStruct>,
}


