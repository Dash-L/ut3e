use crate::error::UT3Error;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Player {
    X,
    O,
}

impl ToString for Player {
    fn to_string(&self) -> String {
        match self {
            Player::X => "X".to_string(),
            Player::O => "O".to_string(),
        }
    }
}

impl TryFrom<&str> for Player {
    type Error = UT3Error;
    fn try_from(string: &str) -> Result<Self, Self::Error> {
        if string == "X" {
            Ok(Self::X)
        } else if string == "O" {
            Ok(Self::O)
        } else {
            Err(UT3Error::InvalidPlayer(string.to_string()))
        }
    }
}

// Maybe this isn't great, but the elements are ordered such that they correctly index into a 1D list of 9 elements
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Direction {
    NW,
    N,
    NE,
    W,
    C,
    E,
    SW,
    S,
    SE,
}

impl Direction {
    pub fn index(&self) -> usize {
        *self as usize
    }
}

impl TryFrom<&str> for Direction {
    type Error = UT3Error;
    fn try_from(string: &str) -> Result<Self, Self::Error> {
        match string {
            "NW" => Ok(Self::NW),
            "N" => Ok(Self::N),
            "NE" => Ok(Self::NE),
            "W" => Ok(Self::W),
            "C" => Ok(Self::C),
            "E" => Ok(Self::E),
            "SW" => Ok(Self::SW),
            "S" => Ok(Self::S),
            "SE" => Ok(Self::SE),
            _ => Err(UT3Error::InvalidDirection(string.to_string())),
        }
    }
}

impl TryFrom<u32> for Direction {
    type Error = UT3Error;
    fn try_from(idx: u32) -> Result<Self, Self::Error> {
        match idx {
            0 => Ok(Self::NW),
            1 => Ok(Self::N),
            2 => Ok(Self::NE),
            3 => Ok(Self::W),
            4 => Ok(Self::C),
            5 => Ok(Self::E),
            6 => Ok(Self::SW),
            7 => Ok(Self::S),
            8 => Ok(Self::SE),
            _ => Err(UT3Error::InvalidDirection(format!("{idx}"))),
        }
    }
}

impl TryFrom<(u32, u32)> for Direction {
    type Error = UT3Error;
    fn try_from(coords: (u32, u32)) -> Result<Self, Self::Error> {
        match (3 * coords.0 + coords.1).try_into() {
            Ok(d) => Ok(d),
            Err(_) => Err(UT3Error::InvalidDirection(format!("{coords:?}"))),
        }
    }
}

pub struct Turn {
    pub turn_number: u32,
    pub player: Player,
    pub coords: (Direction, Direction),
}

impl Turn {
    pub fn new(turn_number: u32, player: Player, coords: (Direction, Direction)) -> Self {
        Self {
            turn_number,
            player,
            coords,
        }
    }
}

impl TryFrom<&str> for Turn {
    type Error = UT3Error;
    fn try_from(string: &str) -> Result<Self, Self::Error> {
        let parts = string.split_ascii_whitespace().collect::<Vec<&str>>();
        let turn_number = parts[0].parse()?;
        let player = parts[1].try_into()?;
        let coords = {
            let coords = parts[2].split("/").collect::<Vec<&str>>();
            (coords[0].try_into()?, coords[1].try_into()?)
        };

        Ok(Self {
            turn_number,
            player,
            coords,
        })
    }
}

fn get_win(board: &[Option<Player>]) -> Option<Player> {
    if board[0].is_some() && board[0] == board[1] && board[1] == board[2] {
        board[0]
    } else if board[3].is_some() && board[3] == board[4] && board[4] == board[5] {
        board[3]
    } else if board[6].is_some() && board[6] == board[7] && board[7] == board[8] {
        board[6]
    } else if board[0].is_some() && board[0] == board[3] && board[3] == board[6] {
        board[0]
    } else if board[1].is_some() && board[1] == board[4] && board[4] == board[7] {
        board[1]
    } else if board[2].is_some() && board[2] == board[5] && board[5] == board[8] {
        board[2]
    } else if board[0].is_some() && board[0] == board[4] && board[4] == board[8] {
        board[0]
    } else if board[2].is_some() && board[2] == board[4] && board[4] == board[6] {
        board[2]
    } else {
        None
    }
}

#[derive(Debug)]
pub struct Box {
    pub winner: Option<Player>,
    inner: [Option<Player>; 9],
}

impl Box {
    pub fn get_tile(&self, idx: Direction) -> &Option<Player> {
        &self.inner[idx.index()]
    }

    pub fn get_tile_mut(&mut self, idx: Direction) -> &mut Option<Player> {
        &mut self.inner[idx.index()]
    }

    pub fn is_full(&self) -> bool {
        self.inner.iter().all(|player| player.is_some())
    }
}

impl Default for Box {
    fn default() -> Self {
        Self {
            winner: None,
            inner: [None; 9],
        }
    }
}

#[derive(Debug)]
pub struct Grid {
    pub winner: Option<Player>,
    pub track: Option<Direction>,
    inner: [Box; 9],
}

impl Grid {
    pub fn apply_turn(&mut self, turn: &Turn) -> Result<(), UT3Error> {
        if self.track.is_some() && turn.coords.0 != self.track.unwrap() {
            Err(UT3Error::WrongTrack {
                required: self.track.unwrap(),
                got: turn.coords.0,
            })
        } else if self
            .get_box(turn.coords.0)
            .get_tile(turn.coords.1)
            .is_some()
        {
            Err(UT3Error::PositionTaken {
                position: turn.coords,
                value: self.get_box(turn.coords.0).get_tile(turn.coords.1).unwrap(),
            })
        } else if self.box_is_finished(turn.coords.0) {
            Err(UT3Error::BoxHasWinner(turn.coords.0))
        } else {
            *self.get_box_mut(turn.coords.0).get_tile_mut(turn.coords.1) = Some(turn.player);

            self.update_wins();

            self.track = if self.box_is_finished(turn.coords.1) {
                None
            } else {
                Some(turn.coords.1)
            };

            Ok(())
        }
    }

    pub fn get_box(&self, idx: Direction) -> &Box {
        &self.inner[idx.index()]
    }

    pub fn get_box_mut(&mut self, idx: Direction) -> &mut Box {
        &mut self.inner[idx.index()]
    }

    pub fn box_is_finished(&self, idx: Direction) -> bool {
        return self.get_box(idx).winner.is_some() || self.get_box(idx).is_full();
    }

    pub fn get_valid_boxes(&self, track: Option<Direction>) -> Vec<(Direction, Direction)> {
        let mut boxes = Vec::new();
        if let Some(track) = track {
            let b = self.get_box(track);
            for i in 0..9 {
                let dir = i.try_into().unwrap();
                if b.get_tile(dir).is_none() {
                    boxes.push((track, dir));
                }
            }
        } else {
            for i in 0..9 {
                let d1 = i.try_into().unwrap();
                let b = self.get_box(d1);
                if b.winner.is_none() && !b.is_full() {
                    for j in 0..9 {
                        let d2 = j.try_into().unwrap();
                        if b.get_tile(d2).is_none() {
                            boxes.push((d1, d2));
                        }
                    }
                }
            }
        }

        boxes
    }

    fn update_wins(&mut self) {
        for mut b in &mut self.inner {
            b.winner = get_win(&b.inner);
        }

        self.winner = get_win(
            &self
                .inner
                .iter()
                .map(|b| b.winner)
                .collect::<Vec<Option<Player>>>(),
        );
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            winner: None,
            track: None,
            inner: Default::default(), // essentially just [Box::default(); 9], but that would require having Copy on Box, which is probably not good
        }
    }
}
