use anchor_lang::prelude::*;

use crate::{constants::*, error::*, instructions::*, states::*, utils::*};
use anchor_spl::{
    associated_token::{self},
    token::{self, Mint, Token, TokenAccount, Transfer},
};

use std::mem::size_of;

#[derive(Accounts)]
#[instruction(arena_id: u64)]
pub struct ClaimReward<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
      seeds = [GLOBAL_STATE_SEED],
      bump,
    )]
    pub global_state: Box<Account<'info, GlobalState>>,

    #[account(
      mut,
      seeds = [ARENA_STATE_SEED, &arena_id.to_le_bytes()],
      bump,
    )]
    pub arena_state: Account<'info, ArenaState>,

    #[account(
      mut,
      seeds = [USER_BET_SEED, user.key().as_ref(), &arena_id.to_le_bytes()],
      bump,
    )]
    pub user_bet_state: Account<'info, UserBetState>,

    #[account(
      mut,
      associated_token::mint = token_mint,
      associated_token::authority = user
    )]
    pub user_ata: Account<'info, TokenAccount>,

    #[account(
      mut,
      associated_token::mint = token_mint,
      associated_token::authority = global_state,
    )]
    pub escrow_ata: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> ClaimReward<'info> {
    fn validate(&self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp as u64;
        // require!(current_time > )

        // check bet result
        require!(
            self.user_bet_state.is_up == self.arena_state.bet_result,
            BettingError::BetResultMisMatch
        );
        // check if user has claimed
        require!(
            self.user_bet_state.is_claimed == 0,
            BettingError::AlreadyClaimed
        );
        Ok(())
    }
    fn claim_reward_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.escrow_ata.to_account_info(),
                to: self.user_ata.to_account_info(),
                authority: self.global_state.to_account_info(),
            },
        )
    }
}

#[access_control(ctx.accounts.validate())]
pub fn handler(ctx: Context<ClaimReward>, arena_id: u64) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp as u64;
    let accts = ctx.accounts;

    let bet_total_amount = accts
        .arena_state
        .up_amount
        .checked_add(accts.arena_state.down_amount)
        .unwrap();

    let platform_fee = (bet_total_amount as u128)
        .checked_mul(accts.global_state.reward_fee_rate as u128)
        .unwrap()
        .checked_div(FEE_RATE_DENOMINATOR as u128)
        .unwrap();
    let total_reward = (bet_total_amount as u128)
        .checked_sub(platform_fee)
        .unwrap();

    // user reward = total * (mybetUp / totalbetInMySide)
    let total_user_success_bet = if accts.arena_state.bet_result == 0 {
        accts.arena_state.down_amount
    } else {
        accts.arena_state.up_amount
    };

    let user_reward = total_reward
        .checked_mul(accts.user_bet_state.bet_amount as u128)
        .unwrap()
        .checked_div(total_user_success_bet as u128)
        .unwrap() as u64;

    let signer_seeds = &[
        GLOBAL_STATE_SEED,
        &[*(ctx.bumps.get("global_state").unwrap())],
    ];
    token::transfer(
        accts.claim_reward_context().with_signer(&[signer_seeds]),
        user_reward,
    )?;
    Ok(())
}