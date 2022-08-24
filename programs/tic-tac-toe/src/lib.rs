pub mod game;

use game::Game;

use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod tic_tac_toe {
    use super::*;

    pub fn setup_game(ctx: Context<SetupGame>, player_two: Pubkey) -> Result<()> {
        ctx.accounts
            .game
            .start([ctx.accounts.player_one.key(), player_two])
    }

    pub fn play(ctx: Context<Play>, tile: crate::game::Tile) -> Result<()> {
        let game = &mut ctx.accounts.game;

        require_keys_eq!(
            game.current_player(),
            ctx.accounts.player.key(),
            crate::game::TicTacToeError::NotPlayersTurn
        );

        game.play(&tile)
    }

}

#[derive(Accounts)]
pub struct SetupGame<'info> {
    #[account(init, payer = player_one, space = 8 + Game::MAXIMUM_SIZE)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player_one: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Play<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    pub player: Signer<'info>,
}
