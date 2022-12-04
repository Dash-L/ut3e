use thiserror::Error;

use crate::game::{Direction, Player};

#[derive(Error, Debug)]
pub enum UT3Error {
    #[error("turn number must be an integer")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("invalid player symbol: `{0}`")]
    InvalidPlayer(String),
    #[error("invalid direction symbol: `{0}`")]
    InvalidDirection(String),
    #[error("wrong track: should have been `{required:?}` but was `{got:?}`")]
    WrongTrack { required: Direction, got: Direction },
    #[error("position taken: attempted to go in position `{position:?}`, which already contains `{value:?}`")]
    PositionTaken {
        position: (Direction, Direction),
        value: Player,
    },
    #[error("box `{0:?}` is finished")]
    BoxHasWinner(Direction),
}
