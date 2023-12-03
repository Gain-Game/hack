use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod staking_rewards {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, rewards_duration: u64) -> Result<()> {
        // Initialization logic would go here
        let staking_rewards = &mut ctx.accounts.staking_rewards;

        // Initialize the rewards duration.
        staking_rewards.rewards_duration = rewards_duration;
        staking_rewards.last_update_time = Clock::get().unwrap().unix_timestamp;

        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: ctx.accounts.staker.to_account_info(),
            to: ctx.accounts.staking_vault.to_account_info(),
            authority: ctx.accounts.staker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        Ok(())
    }

    // Functions 'withdraw', 'get_reward', and 'notify_reward_amount' should be implemented below
}

#[account]
pub struct StakingRewardsState {
    rewards_token: Pubkey,
    staking_token: Pubkey,
    period_finish: i64,
    reward_rate: u64,
    rewards_duration: u64,
    last_update_time: i64,
    reward_per_token_stored: u64,
    total_supply: u64,
    user_state: Vec<UserState>,
}

#[account]
pub struct UserState {
    user: Pubkey,
    amount_staked: u64,
    reward_per_token_paid: u64,
    rewards: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8 + 8 + 8 + 8 + 8 + 8 + 32 + 32 + 32 * 100)]
    pub staking_rewards: Account<'info, StakingRewardsState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,
    #[account(mut)]
    pub staking_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

// Implement other structs for Withdraw, GetReward, and NotifyRewardAmount contexts here

// Modifier equivalent in Anchor would be a function which we can call at the beginning of each method where necessary
fn update_reward<'info>(program_account: &Account<'info, StakingRewardsState>, user_state: &mut Account<'info, UserState>) -> Result<()> {
    // Logic to update the user's reward
    Ok(())
}

