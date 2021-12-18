use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Mint, SetAuthority, TokenAccount, Transfer};

declare_id!("CveUqsUvmZbHuHWNt5hLrVwVjKds3tWwowzC6JpjAiPk");

const n_rounds = 10;

#[program]
pub mod flan_flattery {
    use super::*;
    pub fn initialize_program(ctx: Context<InitializeProgram>) -> ProgramResult {
        // initialize program state - no open games, no waiting players
        Ok(())
    }

    pub fn open_game(ctx: Context<OpenGame>) -> ProgramResult {
        // if none open, open a new game
            // create account for game
            // send tokens to account
        // if one open, join the open game, start game
            // send tokens to account
            // start game
        Ok(())
    }

    pub fn cancel_game(ctx: Context<CancelGame>) -> ProgramResult {
        // make sure given user is a part of an open game
        // return money
        // close game
        Ok(())
    }

    pub fn submit_choice(ctx: Context<SubmitChoice>, choice: bool) -> ProgramResult {
        // if round_num == n_rounds, abort
        // record answer
        // if other account done, increment round_num
        Ok(())
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> ProgramResult {
        // if game not over (round_num != n_rounds) abort
        // calculate winnings for given account based on the game, transfer payout
        // if second, close game account
        Ok(())
    }
}

// Instructions
#[derive(Accounts)]
pub struct InitializeProgram<'info> {

    // create this
    pub program_state: Account<'info, ProgramState>,
    // create this as well
    pub pool_acc: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct OpenGame<'info> {
    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,
    #[account(mut, signer)]
    pub player: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CancelGame<'info> {
    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,
    #[account(mut, signer)]
    pub player: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SubmitChoice<'info> {
    #[account(mut)]
    pub game_acc: Account<'info, Game>,
    #[account(mut, signer)]
    pub player: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub game_acc: Account<'info, Game>,
    #[account(mut)]
    pub pool_acc: Account<'info, TokenAccount>,
    #[account(mut, signer)]
    pub player: AccountInfo<'info>,
}


// Accounts
#[account]
pub struct ProgramState {
    pub game_open: bool,
    pub player_one: Option<Pubkey>,
}

#[account]
pub struct Game {
    pub round_num: u8,
    pub player_one_moves: [u8, n_rounds],
    pub player_two_moves: [u8, n_rounds],
    pub player_one_acc: Pubkey,
    pub player_two_acc: Pubkey,
    pub token_pool: Pubkey,
}