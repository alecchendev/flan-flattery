use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod flan_flattery {
    use super::*;
    pub fn initialize_program(ctx: Context<InitializeProgram>) -> ProgramResult {
        Ok(())
    }

    pub fn open_game(ctx: Context<OpenGame>) -> ProgramResult {
        // if first, open a new game
            //
        // if second, join the open game, start game
        Ok(())
    }

    pub fn cancel_game(ctx: Context<CancelGame>) -> ProgramResult {
        Ok(())
    }

    fn initialize_game(ctx: Context<InitializeGame>) -> ProgramResult {
        Ok(())
    }

    pub fn submit_choice(ctx: Context<SubmitChoice>, choice: bool) -> ProgramResult {
        Ok(())
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeProgram<'info> {

}

#[derive(Accounts)]
pub struct OpenGame<'info> {
    // user token account
    // game token account
}

#[derive(Accounts)]
pub struct CancelGame<'info> {
    
}

#[derive(Accounts)]
pub struct InitializeGame<'info> {
    
}

#[derive(Accounts)]
pub struct SubmitChoice<'info> {
    
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    
}
