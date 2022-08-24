use anchor_lang::prelude::*;
use instructions::*;
use state::*;

mod errors;
mod instructions;
mod state;

declare_id!("91kstpKiSYGKDHzSkWQp6vuUhQJ1jS69EbHHhH2bk8TH");

#[program]
pub mod tic_tac_toe {
    use crate::errors::TicTacToeError;

    use super::*;

    pub fn setup_game(ctx: Context<SetupGame>, player_two: Pubkey) -> Result<()> {
        ctx.accounts
            .game
            .start([ctx.accounts.player_one.key(), player_two])
    }

    pub fn play(ctx: Context<Play>, tile: Tile) -> Result<()> {
        let game = &mut ctx.accounts.game;

        require_keys_eq!(
            game.current_player(),
            ctx.accounts.player.key(),
            TicTacToeError::NotPlayersTurn
        );

        game.play(&tile)
    }
}