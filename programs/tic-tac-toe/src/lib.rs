use anchor_lang::prelude::*;
use instructions::*;
use state::*;

mod errors;
mod instructions;
mod state;

declare_id!("91kstpKiSYGKDHzSkWQp6vuUhQJ1jS69EbHHhH2bk8TH");

#[program]
pub mod tic_tac_toe {
    use super::*;

    pub fn setup_game(ctx: Context<SetupGame>, player_two: Pubkey) -> Result<()> {
        instructions::setup_game::setup_game(ctx, player_two)
    }

    pub fn play(ctx: Context<Play>, tile: Tile) -> Result<()> {
        instructions::play::play(ctx, tile)
    }
}