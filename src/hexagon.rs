use crate::pieces::draw_stones;
use crate::player::Player;
use iced::widget::canvas::{path::Builder, Frame, Path, Stroke};
use iced::{Color, Point, Rectangle};
use sixes_game::PlayerID;
use sixes_game::{Coord, CENTER_HEX};

#[derive(Debug, Clone)]
struct Line {
    start: Point,
    end: Point,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Collinear,
    Clockwise,
    CounterClockwise,
}

#[derive(Debug, Clone)]
pub struct HexagonUI {
    center: Point,
    radius: f32,
    rotation: f32,
    coord: Coord,
}

#[derive(Debug, Clone)]
pub struct HexagonGrid {
    padding_x: f32,
    padding_y: f32,
}

fn direction(p1: Point, p2: Point, p3: Point) -> Direction {
    let val = (p2.y - p1.y) * (p3.x - p2.x) - (p2.x - p1.x) * (p3.y - p2.y);

    if val == 0.0 {
        return Direction::Collinear;
    } else if val > 0.0 {
        return Direction::Clockwise;
    } else {
        return Direction::CounterClockwise;
    }
}

impl Line {
    fn on_line(&self, p: Point) -> bool {
        // Check whether p is on the line or not
        if p.x <= self.start.x.max(self.end.x)
            && p.x <= self.start.x.min(self.end.x)
            && (p.y <= self.start.y.max(self.end.y) && p.y <= self.start.y.min(self.end.y))
        {
            return true;
        }

        false
    }

    fn does_intersect(&self, other: &Line) -> bool {
        let dir1 = direction(self.start, self.end, other.start);
        let dir2 = direction(self.start, self.end, other.end);
        let dir3 = direction(other.start, other.end, self.start);
        let dir4 = direction(other.start, other.end, self.end);

        if dir1 != dir2 && dir3 != dir4 {
            return true;
        }

        if dir1 == Direction::Collinear && self.on_line(other.start) {
            return true;
        }

        if dir2 == Direction::Collinear && self.on_line(other.end) {
            return true;
        }

        if dir3 == Direction::Collinear && other.on_line(self.start) {
            return true;
        }

        if dir4 == Direction::Collinear && other.on_line(self.end) {
            return true;
        }

        false
    }
}

impl HexagonUI {
    fn new(center: Point, radius: f32, rotation: f32, coord: Coord) -> Self {
        HexagonUI {
            center,
            rotation,
            radius,
            coord,
        }
    }

    fn get_coordinates(&self) -> [Point; 6] {
        let mut coordinates = [Point::new(0.0, 0.0); 6];
        let angle_degrees = 60.0;

        for i in 0..6 {
            let angle = (angle_degrees * i as f32 + self.rotation).to_radians();
            let x = self.center.x + self.radius * angle.cos();
            let y = self.center.y + self.radius * angle.sin();

            coordinates[i] = Point::new(x, y);
        }

        coordinates
    }

    pub fn draw(&self) -> Path {
        let mut builder = Builder::new();

        for vertex in self.get_coordinates().iter() {
            if vertex == &self.get_coordinates()[0] {
                builder.move_to(*vertex);
            } else {
                builder.line_to(*vertex);
            }
        }

        builder.close();
        builder.build()
    }

    pub fn contains(&self, point: Point) -> bool {
        let vertices = self.get_coordinates();
        let ex_line = Line {
            start: point,
            end: Point::new(100000.0, point.y),
        };

        let mut count = 0;
        for i in 0..6 {
            let line = Line {
                start: vertices[i],
                end: vertices[(i + 1) % 6],
            };

            if line.does_intersect(&ex_line) {
                if direction(line.start, point, line.end) == Direction::Collinear {
                    return line.on_line(point);
                }

                count += 1;
            }
        }

        count % 2 == 1
    }
}

impl HexagonGrid {
    pub fn new(padding_x: f32, padding_y: f32) -> Self {
        HexagonGrid {
            padding_x,
            padding_y,
        }
    }

    fn create_hexagons(&self, width: f32) -> Vec<HexagonUI> {
        let mut hexagons = Vec::new();

        let hexagon_centers = [
            (1.0, 1.0, Coord::A1),
            (3.0, 1.0, Coord::B1),
            (5.0, 1.0, Coord::C1),
            (7.0, 1.0, Coord::D1),
            (2.0, 5.0 / 2.0, Coord::B2),
            (4.0, 5.0 / 2.0, Coord::C2),
            (6.0, 5.0 / 2.0, Coord::D2),
            (1.0, 4.0, Coord::B3),
            (3.0, 4.0, Coord::C3),
            (5.0, 4.0, Coord::D3),
            (7.0, 4.0, Coord::E3),
        ];

        let hexagon_width = (width - self.padding_x * 2.0) / 8.0;
        let hexagon_height = (hexagon_width / 3.0f32.sqrt()) * 2.0;

        for (x, y, coord) in hexagon_centers.iter() {
            let hexagon = HexagonUI::new(
                Point::new(
                    (x * hexagon_width) + self.padding_x,
                    (y * hexagon_height) + self.padding_y,
                ),
                hexagon_height,
                30.0,
                coord.clone(),
            );

            hexagons.push(hexagon);
        }

        hexagons
    }

    pub fn draw_hexes(&self, bounds: Rectangle) -> Frame {
        let mut frame = Frame::new(bounds.size());
        let hexagons = self.create_hexagons(bounds.width);

        for hexagon in hexagons.iter() {
            if hexagon.coord == CENTER_HEX {
                frame.fill(&hexagon.draw(), Color::from_rgb(1.0, 1.0, 0.8));
            }
            frame.stroke(&hexagon.draw(), Stroke::default().with_width(2.0));
        }

        frame
    }

    pub fn draw_circles(&self, board_width: f32, hexes_selected: Vec<Coord>) -> Vec<Path> {
        let mut paths = Vec::new();
        let hexagons = self.create_hexagons(board_width);

        for hexagon in hexagons.iter() {
            if hexes_selected.contains(&hexagon.coord) {
                paths.push(Path::circle(hexagon.center, hexagon.radius * 0.8));
            }
        }

        paths
    }

    pub fn draw_pieces(
        &self,
        bounds: Rectangle,
        board: Vec<(Coord, u8, bool, Option<PlayerID>)>,
        player_one: Player,
        player_two: Player,
    ) -> Frame {
        let mut frame = Frame::new(bounds.size());
        let hexagons = self.create_hexagons(bounds.width);

        for hexagon in hexagons.iter() {
            let hex_data = board
                .iter()
                .find(|(coord, _, _, _)| coord == &hexagon.coord);

            if let Some(player_id) = hex_data.and_then(|(_, _, _, player_id)| *player_id) {
                let stones = hex_data.map(|(_, stones, _, _)| *stones).unwrap_or(0);
                let king = hex_data.map(|(_, _, king, _)| *king).unwrap_or(false);
                let player_color = if player_id == player_one.id() {
                    player_one.color()
                } else {
                    player_two.color()
                };
                if king {
                    let path = Path::circle(hexagon.center, hexagon.radius / 4.0);
                    frame.fill(&path, player_color);
                    frame.stroke(&path, Stroke::default().with_width(2.0));
                } else if stones > 0 {
                    let paths = draw_stones(stones, hexagon.center, hexagon.radius);
                    for path in paths {
                        frame.fill(&path, player_color);
                        frame.stroke(&path, Stroke::default().with_width(2.0));
                    }
                }
            }
        }

        frame
    }

    pub fn in_hexagon(&self, point: Point, board_width: f32) -> Option<Coord> {
        let hexagons = self.create_hexagons(board_width);
        for hexagon in hexagons.iter() {
            if hexagon.contains(point) {
                return Some(hexagon.coord.clone());
            }
        }
        None
    }
}
