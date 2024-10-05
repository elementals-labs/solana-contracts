mod enums;
mod events;
mod game_accounts;
mod movements;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::keccak::{hashv, Hash};

use enums::*;
use events::*;
use game_accounts::*;

declare_id!("4spD8zfoTFJDDbijBSgxxB8JsXfLx1jSBGx9K73hBgJz");

#[program]
pub mod elementals_battle {

    use super::*;

    pub fn register_to_play(ctx: Context<RegisterPlayer>, team: [Elemental; 3]) -> Result<()> {
        let queue = &mut ctx.accounts.queue;
        let payer = &ctx.accounts.payer;

        let registration = Registration {
            player: payer.key(),
            team,
        };

        queue.players.push(registration);

        emit!(PlayerRegistered {
            player: payer.key(),
        });

        if queue.players.len() == 2 {
            let room_id = queue.last_room_id;
            queue.last_room_id = room_id;

            let player1 = queue.players[0].player;
            let player2 = queue.players[1].player;

            let players = [
                Player {
                    pubkey: player1,
                    current_elemental: 0,
                    team: queue.players[0].team.clone(),
                },
                Player {
                    pubkey: player2,
                    current_elemental: 0,
                    team: queue.players[1].team.clone(),
                },
            ];

            let game_type = "elementals".to_string();

            let cpi_accs = cpi::accounts::CreateGame {
                game: ctx.accounts.game.clone().to_account_info(),
                payer: ctx.accounts.payer.clone(),
                system_program: ctx.accounts.system_program.clone().to_account_info(),
            };

            let game_context = CpiContext::new(ctx.accounts.game.to_account_info(), cpi_accs);

            cpi::create_game(game_context, queue.last_room_id, players, game_type)?;

            queue.last_room_id += 1;
            queue.players = vec![];
        }

        Ok(())
    }

    pub fn create_game(
        ctx: Context<CreateGame>,
        room_id: u128,
        players: [Player; 2],
        game_type: String,
    ) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.user_actions = vec![];
        game.room_id = room_id;
        game.players = players.clone();
        game.status = GameStatus::Playing;
        game.game_type = (*game_type).to_string();
        emit!(GameCreated {
            room_id,
            game_type,
            players
        });
        Ok(())
    }

    /// play is the hash of the play w/ a nonce
    pub fn commit_play(ctx: Context<Playing>, play: [u8; 32]) -> Result<()> {
        let game = &mut ctx.accounts.game;
        if game.status != GameStatus::Playing {
            return Err(error!(GameErrorCode::FinishedGame));
        }

        let player = &mut ctx.accounts.payer;

        let active_player_key: Pubkey = player.key();

        let id = if game.players[0].pubkey == active_player_key {
            0
        } else if game.players[1].pubkey == active_player_key {
            1
        } else {
            return Err(error!(GameErrorCode::IncorrectUser));
        };

        game.hash_buffer[id] = Some(play);

        emit!(HashCommited {
            room_id: game.room_id,
            player: active_player_key,
            hash: play
        });

        Ok(())
    }

    pub fn play_game(ctx: Context<Playing>, play: UserAction, nonce: u8) -> Result<()> {
        let mut game = &mut ctx.accounts.game;

        if game.status != GameStatus::Playing {
            return Err(error!(GameErrorCode::FinishedGame));
        }

        if game.hash_buffer[0].is_none() || game.hash_buffer[1].is_none() {
            return Err(error!(GameErrorCode::WaitingForCommits));
        }

        let mut data_to_hash: Vec<u8> = play.try_to_vec()?;
        data_to_hash.push(nonce);

        let revealed_hash: Hash = hashv(&[&data_to_hash]);

        let player = &mut ctx.accounts.payer;
        let active_player_key: Pubkey = player.key();

        let id = get_user_id(game, active_player_key)?;

        check_if_hash_is_the_commited(&mut game, &revealed_hash, id)?;
        // the hash is ok here, we continue with the game

        game.play_buffer[id] = Some(play.clone());

        if !game.both_players_revealed() {
            return Ok(()); // wait for the other player to play
        };

        // save the play
        game.user_actions.push(play.clone());

        game.execute()?;

        Ok(())
    }
}

fn check_if_hash_is_the_commited(game: &mut Game, hash: &Hash, id: usize) -> Result<()> {
    if game.hash_buffer[id].unwrap() != hash.to_bytes() {
        game.status = GameStatus::Closed;
        emit!(GameUpdated {
            room_id: game.room_id,
            board: game.user_actions.clone(),
            status: "Game closed".to_string()
        });
        return Err(error!(GameErrorCode::InvalidHash));
    }

    Ok(())
}

pub fn get_user_id(game: &Game, player: Pubkey) -> Result<usize> {
    if game.players[0].pubkey == player {
        Ok(0)
    } else if game.players[1].pubkey == player {
        Ok(1)
    } else {
        return Err(error!(GameErrorCode::IncorrectUser));
    }
}
