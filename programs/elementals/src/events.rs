use crate::game_accounts::*;
use anchor_lang::prelude::*;

#[event]
pub struct GameCreated {
    pub room_id: u128,
    pub game_type: String,
    pub players: [Player; 2],
}

#[event]
pub struct GameUpdated {
    pub room_id: u128,
    pub board: Vec<UserAction>,
    pub status: String,
}

#[event]
pub struct HashCommited {
    pub room_id: u128,
    pub player: Pubkey,
    pub hash: [u8; 32],
}

#[event]
pub struct PlayerRegistered {
    pub player: Pubkey,
}

#[event]
pub struct UserPlayRegistered {
    pub room_id: u128,
    pub player: Pubkey,
    pub play: UserAction,
}
