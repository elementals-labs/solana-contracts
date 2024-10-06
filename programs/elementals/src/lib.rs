mod enums;
mod events;
mod game_accounts;
mod movements;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::{hashv, Hash};

use enums::*;
use events::*;
use game_accounts::*;
use movements::{Movement, Status};

declare_id!("AfFGKj5Wxm9FY4ZKZZMYxKRbx7zNYYKMmEUxnLfme2CX");

#[program]
pub mod elementals {

    use super::*;

    pub fn initialize_queue(ctx: Context<InitializeQueue>, name: String) -> Result<()> {
        let queue = &mut ctx.accounts.queue;
        queue.players = Vec::new();
        queue.last_room_id = 0;
        Ok(())
    }

    pub fn register_to_play(ctx: Context<RegisterPlayer>, team: ElementalTeamInput) -> Result<()> {
        let queue = &mut ctx.accounts.queue;
        let player = &ctx.accounts.player;

        let registration = Registration {
            player: player.key(),
            team: team.into(),
        };

        queue.players.push(registration);

        emit!(PlayerRegistered {
            player: player.key(),
        });

        if queue.players.len() == 2 {
            let room_id = queue.last_room_id;
            queue.last_room_id += 1;

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

            // Instead of CPI, we'll create the game directly here
            let game = &mut ctx.accounts.game;
            game.user_actions = Vec::new();
            game.room_id = room_id;
            game.players = players.clone();
            game.status = GameStatus::Playing;
            game.game_type = game_type.clone();

            emit!(GameCreated {
                room_id,
                game_type,
                players
            });

            queue.players.clear();
        }

        Ok(())
    }
    /*  I COULD NOT PASS A HASHED BORSH SERIALIZED STRUCT TO THE PROGRAM. yet.
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
    */
    pub fn play_game(ctx: Context<Playing>, play: UserAction /* , nonce: u8*/) -> Result<()> {
        let game = &mut ctx.accounts.game;

        if game.status != GameStatus::Playing {
            return Err(error!(GameErrorCode::FinishedGame));
        }

        let player = &mut ctx.accounts.payer;
        let active_player_key: Pubkey = player.key();

        let id = get_user_id(game, active_player_key)?;

        /* This is commit related code, I could not pass the hash to the program yet.
                if game.hash_buffer[0].is_none() || game.hash_buffer[1].is_none() {
                    return Err(error!(GameErrorCode::WaitingForCommits));
                }

                let mut data_to_hash: Vec<u8> = play.try_to_vec()?;
                data_to_hash.push(nonce);

                let revealed_hash: Hash = hashv(&[&data_to_hash]);

                check_if_hash_is_the_commited(&mut game, &revealed_hash, id)?;
                // the hash is ok here, we continue with the game
        */
        game.play_buffer[id] = Some(play.clone());

        emit!(UserPlayRegistered {
            room_id: game.room_id,
            player: active_player_key,
            play: play.clone()
        });

        if !game.both_players_revealed() {
            return Ok(()); // wait for the other player to play
        };

        // save the play
        game.user_actions.push(play.clone());

        game.execute()?;

        Ok(())
    }
}

#[allow(dead_code)]
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

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub struct ElementalTeamInput {
    elementals: [ElementalInput; 3],
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub struct ElementalInput {
    name: String,
    stats: Stats,
    movements: [Movement; 4],
    is_alive: bool,
    status: Status,
}

impl Into<[Elemental; 3]> for ElementalTeamInput {
    fn into(self) -> [Elemental; 3] {
        self.elementals.map(|input| Elemental {
            name: input.name,
            stats: input.stats.clone(),
            starting_stats: input.stats.clone(),
            movements: input.movements.map(|movement| movement.get_info()),
            is_alive: input.is_alive,
            status: input.status,
        })
    }
}
