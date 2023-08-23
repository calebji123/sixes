use iced::Color;
use sixes_game::PlayerID;

#[derive(Debug, Clone, Copy)]
pub struct Player {
    id: PlayerID,
    stones: u8,
    king: bool,
    color: Color,
}

impl Player {
    pub fn new(data: (PlayerID, u8, bool)) -> Self {
        Player {
            id: data.0,
            stones: data.1,
            king: data.2,
            color: match data.0 {
                PlayerID::One => Color::from_rgb8(0xFF, 0xFF, 0xFF),
                PlayerID::Two => Color::from_rgb8(0x00, 0x00, 0x00),
            },
        }
    }

    pub fn update(&mut self, data: (PlayerID, u8, bool)) {
        self.id = data.0;
        self.stones = data.1;
        self.king = data.2;
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

    pub fn color(&self) -> Color {
        self.color
    }
}
