use crate::player::PlayerID;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Coord {
    A1,
    B1,
    B2,
    B3,
    C1,
    C2,
    C3,
    D1,
    D2,
    D3,
    E3,
}

#[derive(Debug, Clone)]
pub struct Board {
    hexes: Vec<Hex>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hex {
    coord: Coord,
    stone: u8,
    king: bool,
    player: Option<PlayerID>,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Triple {
    a: Coord,
    b: Coord,
    c: Coord,
    player: PlayerID,
}

impl Triple {
    pub fn player(&self) -> PlayerID {
        self.player
    }
}

impl Hex {
    pub fn new(coord: Coord) -> Self {
        Hex {
            coord,
            stone: 0,
            king: false,
            player: None,
        }
    }

    pub fn coord(&self) -> Coord {
        self.coord
    }

    pub fn stone(&self) -> u8 {
        self.stone
    }

    pub fn king(&self) -> bool {
        self.king
    }

    pub fn player(&self) -> Option<PlayerID> {
        self.player
    }

    pub fn increment_stone(&mut self) {
        self.stone += 1;
    }

    pub fn set_king(&mut self) {
        self.king = true;
    }

    pub fn set_player(&mut self, player: PlayerID) {
        self.player = Some(player);
    }

    pub fn play_stone(&mut self, player: PlayerID) {
        self.increment_stone();
        self.set_player(player);
    }

    pub fn play_king(&mut self, player: PlayerID) {
        self.set_king();
        self.set_player(player);
    }
}

impl Board {
    pub fn new() -> Self {
        Board {
            hexes: vec![
                Hex::new(Coord::A1),
                Hex::new(Coord::B1),
                Hex::new(Coord::B2),
                Hex::new(Coord::B3),
                Hex::new(Coord::C1),
                Hex::new(Coord::C2),
                Hex::new(Coord::C3),
                Hex::new(Coord::D1),
                Hex::new(Coord::D2),
                Hex::new(Coord::D3),
                Hex::new(Coord::E3),
            ],
        }
    }

    pub fn hexes(&self) -> &Vec<Hex> {
        &self.hexes
    }

    pub fn hex(&self, coord: Coord) -> &Hex {
        self.hexes
            .iter()
            .find(|hex| hex.coord == coord)
            .expect("Hex not found")
    }

    pub fn hex_mut(&mut self, coord: Coord) -> &mut Hex {
        self.hexes
            .iter_mut()
            .find(|hex| hex.coord == coord)
            .expect("Hex not found")
    }

    pub fn play_stone(&mut self, coord: Coord, player: PlayerID) {
        let hex = self.hex_mut(coord);
        hex.play_stone(player);
    }

    pub fn play_king(&mut self, coord: Coord, player: PlayerID) {
        let hex = self.hex_mut(coord);
        hex.play_king(player);
    }

    //scores if a player gets 3 in a row in this hexagonal grid
    pub fn same_player(&self, coord1: Coord, coord2: Coord, coord3: Coord) -> bool {
        let hex1 = self.hex(coord1);
        let hex2 = self.hex(coord2);
        let hex3 = self.hex(coord3);

        if let Some(player1) = hex1.player {
            if let Some(player2) = hex2.player {
                if let Some(player3) = hex3.player {
                    if player1 == player2 && player2 == player3 {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn score(&mut self) -> Vec<Triple> {
        let triples = vec![
            (Coord::A1, Coord::B1, Coord::C1),
            (Coord::B1, Coord::C1, Coord::D1),
            (Coord::B1, Coord::B2, Coord::B3),
            (Coord::C1, Coord::C2, Coord::C3),
            (Coord::D1, Coord::D2, Coord::D3),
            (Coord::B2, Coord::C2, Coord::D2),
            (Coord::B3, Coord::C3, Coord::D3),
            (Coord::C3, Coord::D3, Coord::E3),
            (Coord::A1, Coord::B2, Coord::C3),
            (Coord::B1, Coord::C2, Coord::D3),
            (Coord::C1, Coord::D2, Coord::E3),
        ];

        let mut triples_acc = Vec::new();

        for (coord1, coord2, coord3) in triples {
            if self.same_player(coord1, coord2, coord3) {
                if let Some(player) = self.hex(coord1).player {
                    triples_acc.push(Triple {
                        a: coord1,
                        b: coord2,
                        c: coord3,
                        player: player,
                    });
                }
            }
        }

        triples_acc
    }
}
