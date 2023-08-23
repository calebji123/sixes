mod board;
mod player;

pub use crate::board::Coord;
use crate::board::{Board, Triple};
use crate::player::Player;
pub use crate::player::PlayerID;
pub const CENTER_HEX: Coord = Coord::C2;

#[derive(Debug, Clone)]
pub struct Sixes {
    board: Board,
    player_one: Player,
    player_two: Player,
    turn: PlayerID,
    last_captured: Option<Coord>,
    last_scored: Vec<Triple>,
    game_state: GameState,
}

#[derive(Debug, Clone)]
pub enum Play {
    Stones,
    King,
}

#[derive(Debug, Clone)]
pub enum GameError {
    InvalidPlay,
    InvalidWin, // If somehow both players win at the same time
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameState {
    Running,
    Win(PlayerID),
}

impl Sixes {
    pub fn start() -> Self {
        Sixes {
            board: Board::new(),
            player_one: Player::new(PlayerID::One),
            player_two: Player::new(PlayerID::Two),
            turn: PlayerID::One,
            last_captured: None,
            last_scored: Vec::new(),
            game_state: GameState::Running,
        }
    }

    pub fn reset(&mut self) {
        self.board = Board::new();
        self.player_one = Player::new(PlayerID::One);
        self.player_two = Player::new(PlayerID::Two);
        self.turn = PlayerID::One;
        self.last_captured = None;
        self.last_scored = Vec::new();
        self.game_state = GameState::Running;
    }

    pub fn play(&mut self, coord: Coord, play: Play) -> Result<(), GameError> {
        if self.game_state != GameState::Running {
            return Err(GameError::InvalidPlay);
        }
        if self.check_playable(coord, play.clone()) == false {
            return Err(GameError::InvalidPlay);
        }
        let hex = self.board.hex(coord);

        match play {
            Play::Stones => {
                match hex.player() {
                    Some(player_id) => {
                        match player_id {
                            PlayerID::One => self.player_one.add_to_graveyard(hex.stone()),
                            PlayerID::Two => self.player_two.add_to_graveyard(hex.stone()),
                        };
                    }
                    None => {
                        if hex.coord() != CENTER_HEX {
                            match self.turn {
                                PlayerID::One => self.player_one.add_stones(2),
                                PlayerID::Two => self.player_two.add_stones(2),
                            };
                        }
                    }
                }

                match self.turn {
                    PlayerID::One => self.player_one.remove_stones(hex.stone() + 1),
                    PlayerID::Two => self.player_two.remove_stones(hex.stone() + 1),
                };
                self.board.play_stone(coord, self.turn);
            }
            Play::King => {
                match hex.player() {
                    Some(player_id) => {
                        match player_id {
                            PlayerID::One => self.player_one.add_to_graveyard(hex.stone()),
                            PlayerID::Two => self.player_two.add_to_graveyard(hex.stone()),
                        };
                    }
                    None => {}
                }

                match self.turn {
                    PlayerID::One => self.player_one.play_king(),
                    PlayerID::Two => self.player_two.play_king(),
                };
                self.board.play_king(coord, self.turn);
            }
        }

        self.last_captured = Some(coord);
        self.score()?;
        self.next_turn();
        Result::Ok(())
    }

    fn check_playable(&self, coord: Coord, play: Play) -> bool {
        let player = match self.turn {
            PlayerID::One => &self.player_one,
            PlayerID::Two => &self.player_two,
        };

        let hex = self.board.hex(coord);

        match self.last_captured {
            Some(last_caputred) => {
                if last_caputred == coord {
                    return false;
                }
            }
            None => {}
        }

        match play {
            Play::Stones => {
                if let Some(player_id) = hex.player() {
                    if player_id != player.id()
                        && player.stones() > hex.stone()
                        && hex.king() == false
                    {
                        return true;
                    }
                    return false;
                }
                true
            }
            Play::King => {
                if let Some(player_id) = hex.player() {
                    if player_id != player.id()
                        && player.king()
                        && hex.stone() > 0
                        && hex.king() == false
                        && hex.coord() != CENTER_HEX
                    {
                        return true;
                    }
                }
                false
            }
        }
    }

    fn score(&mut self) -> Result<(), GameError> {
        let new_score = self.board.score();
        let mut stay_scored = Vec::new();
        for triple1 in new_score.clone() {
            for triple2 in self.last_scored.clone() {
                if triple1 == triple2 {
                    stay_scored.push(triple1.player());
                }
            }
        }
        self.last_scored = new_score.clone();
        // if all of stay_scored is the same player return ok
        if stay_scored
            .iter()
            .all(|item| item.eq(stay_scored.first().unwrap_or(&item)))
        {
            if stay_scored.len() > 0 {
                self.game_state = GameState::Win(stay_scored[0]);
            }
            Result::Ok(())
        } else {
            Result::Err(GameError::InvalidWin)
        }
    }

    fn next_turn(&mut self) {
        match self.turn {
            PlayerID::One => self.turn = PlayerID::Two,
            PlayerID::Two => self.turn = PlayerID::One,
        }
    }
}

// Data retrieval functions
impl Sixes {
    pub fn board(&self) -> Vec<(Coord, u8, bool, Option<PlayerID>)> {
        self.board
            .hexes()
            .iter()
            .map(|hex| (hex.coord().clone(), hex.stone(), hex.king(), hex.player()))
            .collect()
    }

    pub fn players(&self) -> Vec<(PlayerID, u8, bool)> {
        vec![
            (
                self.player_one.id(),
                self.player_one.stones(),
                self.player_one.king(),
            ),
            (
                self.player_two.id(),
                self.player_two.stones(),
                self.player_two.king(),
            ),
        ]
    }

    pub fn game_state(&self) -> GameState {
        self.game_state.clone()
    }

    pub fn possible_moves(&self) -> Vec<(Coord, Play)> {
        let mut possible_moves = Vec::new();
        for hex in self.board.hexes() {
            if self.check_playable(hex.coord(), Play::Stones) {
                possible_moves.push((hex.coord(), Play::Stones));
            }
            if self.check_playable(hex.coord(), Play::King) {
                possible_moves.push((hex.coord(), Play::King));
            }
        }
        possible_moves
    }

    pub fn turn(&self) -> PlayerID {
        self.turn.clone()
    }
}
