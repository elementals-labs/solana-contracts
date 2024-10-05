use anchor_lang::prelude::*;

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum GameStatus {
    Playing,
    Winner(Pubkey),
    Closed,
}

#[error_code]
pub enum GameErrorCode {
    #[msg("Isn't your turn to play")]
    IncorrectUser,
    #[msg("You can't use this cell now")]
    InvalidCell,
    #[msg("You can't play, this game status is ended")]
    FinishedGame,
    #[msg("Wait until the other player commits his play")]
    WaitingForCommits,
    #[msg("Invalid hash")]
    InvalidHash,
    #[msg("Not enough PP")]
    NotEnoughPP,
}
