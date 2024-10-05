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
    pub elemental: usize,
    pub movement: usize,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub struct Player {
    pub pubkey: Pubkey,
    pub current_elemental: usize,
    pub team: [Elemental; 3],
}

// team[nth_elemental][nth_movement] = info del movement

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub struct Elemental {
    pub name: String,
    pub stats: Stats,
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

        let player1 = &self.players[0].team[self.players[0].current_elemental].movements
            [play1.movement as usize];

        let player2 = &self.players[1].team[self.players[1].current_elemental].movements
            [play2.movement as usize];

        // version easy
        if player1.pp <= 0 || player2.pp <= 0 {
            return Err(error!(GameErrorCode::NotEnoughPP));
        }

        if spe1 > spe2 {
            // self.run_action(player1, 0)?;
            // self.run_action(player2, 1)?;
        } else {
            // self.run_action(player2, 1)?;
            // self.run_action(player1, 0)?;
        }

        Ok(())
    }

    fn run_action(&mut self, movement: MovementInfo, id: u8) -> Result<()> {
        if movement.accuracy.is_some() && random() > movement.accuracy.unwrap() as i8 {
            return Ok(());
        }

        let dmg = dmg_formula(movement);

        let target = (id + 1) % 2;

        self.do_dmg_to_player(dmg, target)?;

        // accuracy
        // si la accuracy del jugador es != None y accuracy <  random -> miss = False/True
        // si move_choice == DeterministicMoveChoice
        // miss = move_choice.miss // pero si la calculo antes para que quiero pisarla con esto?
        // if miss true:
        // pierde el movimiento

        // damage
        // dmg = calcula el damage con una formula -> funcion aparte
        // le resta el dmg al otrole
        // esta el dmg al otro jugador
        Ok(())
    }

    fn do_dmg_to_player(&mut self, dmg: u8, target: u8) -> Result<()> {
        let player = &mut self.players[target as usize];
        let elemental = &mut player.team[player.current_elemental];

        elemental.stats.hp = elemental.stats.hp.saturating_sub(dmg as u8);

        if elemental.stats.hp == 0 {
            elemental.is_alive = false;
        }

        Ok(())
    }
}

fn random() -> i8 {
    4
}
fn dmg_formula(movement: MovementInfo) -> u8 {
    // u128 dmg = floor(  0.75 * accuracy *  power + 1 )

    let power = if movement.power.is_some() {
        movement.power.unwrap()
    } else {
        return 0;
    };

    let accuracy = if movement.accuracy.is_some() {
        movement.accuracy.unwrap()
    } else {
        100
    };

    ((75 * accuracy as u128 * power as u128 + 1) / 100) as u8
}
