use crate::{enums::*, get_user_id, movements::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateGame<'info> {
    #[account(
        init,
        payer = payer,
        space = 900,
        seeds = [b"game".as_ref(), name.as_ref()],
        bump
    )]
    pub game: Account<'info, Game>,
    #[account(mut)]
    /// CHECK:
    pub payer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterPlayer<'info> {
    /// CHECK:
    pub payer: AccountInfo<'info>,
    pub queue: Account<'info, Queue>,
    pub system_program: Program<'info, System>,
    /// CHECK:
    pub game: AccountInfo<'info>,
}

#[account]
pub struct Queue {
    pub players: Vec<Registration>,
    pub last_room_id: u128,
}

#[derive(Accounts)]
pub struct Playing<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    /// CHECK:
    pub payer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Game {
    pub room_id: u128,
    pub game_type: String,
    pub user_actions: Vec<UserAction>,
    pub hash_buffer: [Option<[u8; 32]>; 2],
    pub play_buffer: [Option<UserAction>; 2],
    pub players: [Player; 2],
    pub turn: u8,
    pub status: GameStatus,
}
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub struct Registration {
    pub player: Pubkey,
    pub team: [Elemental; 3],
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub struct UserAction {
    pub player: Pubkey,
    pub elemental: u8,
    pub movement: u8,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub struct Player {
    pub pubkey: Pubkey,
    pub current_elemental: u8,
    pub team: [Elemental; 3],
}

// team[nth_elemental][nth_movement] = info del movement

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub struct Elemental {
    pub name: String,
    pub stats: Stats,
    pub starting_stats: Stats,
    pub movements: [MovementInfo; 4],
    pub is_alive: bool,
    pub status: Status,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub struct Stats {
    pub hp: u8,
    pub atk: u8,
    pub def: u8,
    pub spa: u8, // Special Attack
    pub spd: u8, // Special Defense
    pub spe: u8, // Speed
}

impl UserAction {
    pub fn get_elemental(&self, game: &Game) -> Elemental {
        game.players[get_user_id(game, self.player).unwrap()].team[self.elemental as usize].clone()
    }

    pub fn get_movement(&self, game: &Game) -> MovementInfo {
        self.get_elemental(game).movements[self.movement as usize].clone()
    }
}

impl Game {
    pub fn both_players_revealed(&self) -> bool {
        self.play_buffer[0].is_some() && self.play_buffer[1].is_some()
    }

    pub fn execute(&mut self) -> Result<()> {
        let (play1, play2) = (
            self.play_buffer[0].as_ref().unwrap(),
            self.play_buffer[1].as_ref().unwrap(),
        );

        let (spe1, spe2) = (
            play1.get_elemental(&self).stats.spe,
            play2.get_elemental(&self).stats.spe,
        );

        let player1_info = (
            0 as usize,
            self.players[0].current_elemental as usize,
            play1.movement as usize,
        );
        let player2_info = (
            1 as usize,
            self.players[1].current_elemental as usize,
            play2.movement as usize,
        );

        // Check PP
        if self.players[0].team[player1_info.1].movements[player1_info.2].pp <= 0
            || self.players[1].team[player2_info.1 as usize].movements[player2_info.2].pp <= 0
        {
            return Err(error!(GameErrorCode::NotEnoughPP));
        }

        if spe1 > spe2 {
            self.run_action(player1_info)?;
            self.run_action(player2_info)?;
        } else {
            self.run_action(player2_info)?;
            self.run_action(player1_info)?;
        }

        Ok(())
    }

    fn run_action(&mut self, info: (usize, usize, usize)) -> Result<()> {
        let (id, elemental_index, movement_index) = info;
        let accuracy = self.players[id].team[elemental_index].movements[movement_index].accuracy;
        // let power = self.players[id].team[elemental_index].movements[movement_index].power;
        let effect = self.players[id].team[elemental_index].movements[movement_index]
            .effect
            .clone();

        if accuracy.is_some() && random() > accuracy.unwrap() as i8 {
            return Ok(());
        }

        let target = (id + 1) % 2; // the other player is the target

        self.handle_effects(effect, id, target, accuracy)?;

        Ok(())
    }

    fn do_dmg_to_player(&mut self, acc: u8, dmg: u8, target: usize) -> Result<()> {
        let calculated_dmg = dmg_formula(acc, dmg);

        let player = &mut self.players[target as usize];
        let elemental = &mut player.team[player.current_elemental as usize];

        elemental.stats.hp = elemental.stats.hp.saturating_sub(calculated_dmg as u8);

        if elemental.stats.hp == 0 {
            elemental.is_alive = false;
        }

        Ok(())
    }

    fn heal_player(&mut self, amount: u8, player: usize) -> Result<()> {
        let player = &mut self.players[player];
        let elemental = &mut player.team[player.current_elemental as usize];

        let heal = elemental.stats.hp.saturating_add(amount);

        elemental.stats.hp = if heal > elemental.starting_stats.hp {
            elemental.starting_stats.hp
        } else {
            heal
        };

        Ok(())
    }

    fn handle_effects(
        &mut self,
        effect: Option<Effect>,
        player: usize,
        other_player: usize,
        accuracy: Option<u8>,
    ) -> Result<()> {
        let Some(effect) = effect else {
            return Ok(());
        };

        let accuracy = if accuracy.is_some() {
            accuracy.unwrap()
        } else {
            100
        };

        use Effect::*;
        match effect {
            Damage { amount } => self.do_dmg_to_player(accuracy, amount, other_player)?,
            Heals { amount } => self.heal_player(amount, player)?,
            StatusCondition { condition } => {
                self.players[other_player].team
                    [self.players[other_player].current_elemental as usize]
                    .status = condition
            }
            SelfStatModifier { stat, .. } => {
                let player = &mut self.players[player];
                let _elemental = &mut player.team[player.current_elemental as usize];

                match stat {
                    crate::movements::Stats::Special => todo!(),
                    crate::movements::Stats::Speed => todo!(),
                }
            }
            OpponentStatModifier { stat, .. } => {
                let player = &mut self.players[other_player];
                let _elemental = &mut player.team[player.current_elemental as usize];

                match stat {
                    crate::movements::Stats::Special => todo!(),
                    crate::movements::Stats::Speed => todo!(),
                }
            }
            HighCriticalHitRatio => todo!(),
            HealAndStatusCondition { amount, condition } => {
                self.heal_player(amount, player)?;
                self.players[player].team[self.players[player].current_elemental as usize].status =
                    condition
            }
            Recharge => { /* Someday */ }
            SelfDestruct => {
                self.players[player].team[self.players[player].current_elemental as usize]
                    .is_alive = false
            }
            ChangeElemental { elemental } => self.players[player].current_elemental = elemental,
        };

        Ok(())
    }
}

fn random() -> i8 {
    4
}

fn dmg_formula(accuracy: u8, power: u8) -> u8 {
    ((75 * accuracy as u128 * power as u128 + 1) / 100) as u8
}
