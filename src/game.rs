use std::ops::Add;

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
    NW = 0,
    N = 1,
    NE = 2,
    W = 3,
    C = 4,
    E = 5,
    SW = 6,
    S = 7,
    SE = 8,
}

impl Direction {
    pub fn index(&self) -> usize {
        *self as usize
    }
}

impl Add<Direction> for Direction {
    type Output = Direction;

    fn add(self, rhs: Direction) -> Self::Output {
        let lhs_coords: (u32, u32) = self.into();
        let rhs_coords: (u32, u32) = rhs.into();

        (
            (lhs_coords.0 as i32 + rhs_coords.0 as i32 - 1i32).rem_euclid(3) as u32,
            (lhs_coords.1 as i32 + rhs_coords.1 as i32 - 1i32).rem_euclid(3) as u32,
        )
            .try_into()
            .unwrap()
    }
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::NW => String::from("NW"),
            Direction::N => String::from("N"),
            Direction::NE => String::from("NE"),
            Direction::W => String::from("W"),
            Direction::C => String::from("C"),
            Direction::E => String::from("E"),
            Direction::SW => String::from("SW"),
            Direction::S => String::from("S"),
            Direction::SE => String::from("SE"),
        }
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
        match (3 * coords.1 + coords.0).try_into() {
            Ok(d) => Ok(d),
            Err(_) => Err(UT3Error::InvalidDirection(format!("{coords:?}"))),
        }
    }
}

impl From<Direction> for (u32, u32) {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::NW => (0, 0),
            Direction::N => (1, 0),
            Direction::NE => (2, 0),
            Direction::W => (0, 1),
            Direction::C => (1, 1),
            Direction::E => (2, 1),
            Direction::SW => (0, 2),
            Direction::S => (1, 2),
            Direction::SE => (2, 2),
        }
    }
}

#[derive(Debug)]
pub struct Turn {
    pub turn_number: u32,
    player: Player,
    pub coords: (Direction, Direction),
}

impl Turn {
    pub fn new(turn_number: u32, coords: (Direction, Direction)) -> Self {
        Self {
            turn_number,
            player: if turn_number % 2 == 0 {
                Player::O
            } else {
                Player::X
            },
            coords,
        }
    }
}

impl ToString for Turn {
    fn to_string(&self) -> String {
        format!(
            "{}\t{}\t{}/{}",
            self.turn_number,
            self.player.to_string(),
            self.coords.0.to_string(),
            self.coords.1.to_string()
        )
    }
}

impl TryFrom<&str> for Turn {
    type Error = UT3Error;
    fn try_from(string: &str) -> Result<Self, Self::Error> {
        let parts = string.split_ascii_whitespace().collect::<Vec<&str>>();
        let turn_number = parts[0].parse()?;
        let coords = {
            let coords = parts[2].split("/").collect::<Vec<&str>>();
            (coords[0].try_into()?, coords[1].try_into()?)
        };

        Ok(Self::new(turn_number, coords))
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

#[derive(Copy, Clone, Debug)]
pub enum Variant {
    Relative,
    Absolute,
}

#[derive(Debug)]
pub struct Grid {
    pub winner: Option<Player>,
    pub variant: Variant,
    pub current_turn_number: u32,
    pub turns: Vec<Turn>,
    inner: [Box; 9],
}

impl Grid {
    pub fn apply_turn(&mut self, coords: (Direction, Direction)) -> Result<(), UT3Error> {
        if self.get_track().is_some() && coords.0 != self.get_track().unwrap() {
            Err(UT3Error::WrongTrack {
                required: self.get_track().unwrap(),
                got: coords.0,
            })
        } else if self.get_box(coords.0).get_tile(coords.1).is_some() {
            Err(UT3Error::PositionTaken {
                position: coords,
                value: self.get_box(coords.0).get_tile(coords.1).unwrap(),
            })
        } else if self.box_is_finished(coords.0) {
            Err(UT3Error::BoxHasWinner(coords.0))
        } else {
            let turn = Turn::new(self.current_turn_number, coords);

            *self.get_box_mut(coords.0).get_tile_mut(coords.1) = Some(turn.player);

            self.turns.push(turn);

            self.update_wins();

            self.current_turn_number += 1;
            Ok(())
        }
    }

    pub fn get_track(&self) -> Option<Direction> {
        if let Some(prev_turn) = self.turns.last() {
            let absolute_direction = match self.variant {
                Variant::Relative => prev_turn.coords.0 + prev_turn.coords.1,
                Variant::Absolute => prev_turn.coords.1,
            };

            if self.box_is_finished(absolute_direction) {
                None
            } else {
                Some(absolute_direction)
            }
        } else {
            None
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

    pub fn with_variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }
}

impl ToString for Grid {
    fn to_string(&self) -> String {
        self.turns
            .iter()
            .map(|turn| turn.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            winner: None,
            variant: Variant::Absolute,
            current_turn_number: 1,
            turns: Vec::new(),
            inner: Default::default(), // essentially just [Box::default(); 9], but that would require having Copy on Box, which is probably not good
        }
    }
}
