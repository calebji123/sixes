pub const STARTING_STONES: u8 = 12;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum PlayerID {
    One,
    Two,
}

#[derive(Debug, Clone)]
pub struct Player {
    id: PlayerID,
    stones: u8,
    king: bool,
    graveyard: u8, //deprecated feature (might make a comeback)
}

impl Player {
    pub fn new(id: PlayerID) -> Self {
        Player {
            id,
            stones: STARTING_STONES,
            king: true,
            graveyard: 0,
        }
    }

    pub fn id(&self) -> PlayerID {
        self.id.clone()
    }

    pub fn stones(&self) -> u8 {
        self.stones
    }

    pub fn king(&self) -> bool {
        self.king
    }

    #[allow(dead_code)]
    pub fn graveyard(&self) -> u8 {
        self.graveyard
    }

    pub fn add_stones(&mut self, stones: u8) {
        self.stones += stones;
    }

    pub fn remove_stones(&mut self, stones: u8) {
        self.stones -= stones;
        if self.stones == 0 {
            self.stones = self.graveyard;
            self.clear_graveyard()
        }
    }

    pub fn add_to_graveyard(&mut self, stones: u8) {
        self.graveyard += stones;
    }

    pub fn clear_graveyard(&mut self) {
        self.graveyard = 0;
    }

    pub fn play_king(&mut self) {
        self.king = false;
    }
}
