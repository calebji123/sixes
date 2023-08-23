use iced::widget::canvas::event::{self, Event};
use iced::widget::canvas::{Canvas, Cursor, Frame, Geometry, Program, Stroke};
use iced::widget::{button, column, row, text};
use iced::{mouse, Padding};
use iced::{theme, Alignment, Application, Color, Command, Element, Length};

use sixes_game;
mod hexagon;
mod pieces;
mod player;
use hexagon::HexagonGrid;
use player::Player;
use sixes_game::Coord;
use sixes_game::PlayerID;
use sixes_game::{GameError, GameState, Play, Sixes};

pub fn main() -> iced::Result {
    SixesUI::run(iced::Settings {
        window: iced::window::Settings {
            size: (1000, 800),
            ..iced::window::Settings::default()
        },
        antialiasing: true,
        ..iced::Settings::default()
    })
}

#[derive(Debug, Clone)]
enum SixesUI {
    Menu,
    Running(State),
}

#[derive(Debug, Clone)]
struct State {
    game: Sixes,
    player_one: Player,
    player_two: Player,
    hexes_selectable: Vec<Coord>,
    stone_selected: bool,
    king_selected: bool,
    over_hex: Option<Coord>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Start,
    Menu,
    SelectStone,
    SelectKing,
    SelectHex,
    OverHex(Option<Coord>),
    EndTurn,
    EndGame,
}

impl Application for SixesUI {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = theme::Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (SixesUI::Menu, Command::none())
    }

    fn title(&self) -> String {
        String::from("Sixes")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match self {
            SixesUI::Menu => match message {
                Message::Start => {
                    let game = Sixes::start();
                    let players = game.players();
                    *self = SixesUI::Running(State {
                        game,
                        player_one: Player::new(players[0]),
                        player_two: Player::new(players[1]),
                        hexes_selectable: Vec::new(),
                        stone_selected: false,
                        king_selected: false,
                        over_hex: None,
                    });
                }
                _ => {}
            },
            SixesUI::Running(state) => match message {
                Message::SelectStone => {
                    if !state.stone_selected {
                        println!("Selecting stone");
                        state.stone_selected = true;
                        state.king_selected = false;
                        state.hexes_selectable = state
                            .game
                            .possible_moves()
                            .iter()
                            .filter(|(_, play)| match play {
                                Play::Stones => true,
                                _ => false,
                            })
                            .map(|(coord, _)| coord.clone())
                            .collect();
                    } else {
                        println!("Deselecting stone");
                        state.stone_selected = false;
                        state.hexes_selectable = Vec::new();
                    }
                }
                Message::SelectKing => {
                    if !state.king_selected {
                        println!("Selecting king");
                        state.king_selected = true;
                        state.stone_selected = false;
                        state.hexes_selectable = state
                            .game
                            .possible_moves()
                            .iter()
                            .filter(|(_, play)| match play {
                                Play::King => true,
                                _ => false,
                            })
                            .map(|(coord, _)| coord.clone())
                            .collect();
                    } else {
                        println!("Deselecting king");
                        state.king_selected = false;
                        state.hexes_selectable = Vec::new();
                    }
                }
                Message::SelectHex => {
                    if state.over_hex.is_none() {
                        return Command::none();
                    }
                    let coord = state.over_hex.unwrap();
                    println!("Selecting hex {:?}", coord);
                    let res = if state.stone_selected {
                        state.game.play(coord, Play::Stones)
                    } else if state.king_selected {
                        state.game.play(coord, Play::King)
                    } else {
                        Err(GameError::InvalidPlay)
                    };

                    match res {
                        Ok(_) => {
                            return Command::perform(async {}, move |_| Message::EndTurn);
                        }
                        Err(_) => {}
                    }
                }
                Message::OverHex(coord) => {
                    state.over_hex = coord;
                }
                Message::EndTurn => {
                    println!("ending turn");
                    state.hexes_selectable = Vec::new();
                    state.stone_selected = false;
                    state.king_selected = false;
                    let players = state.game.players();
                    state.player_one.update(players[0]);
                    state.player_two.update(players[1]);
                }
                Message::Menu => {
                    *self = SixesUI::Menu;
                }
                _ => {}
            },
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        match self {
            SixesUI::Menu => column!(
                text("Sixes").size(50),
                column!(
                        text("Rules").size(32), 
                        text("Objective: Maintain a three in a row for two turns straight."),
                        text("On your turn you may:"),
                        text("1. Play a stone on an empty hex. If this hex is not the center hex you get two stones back."),
                        text("2. Play n + 1 stones on an occupied hex with n stones of the opposite colour. The other stones are removed from the board."),
                        text("3. Play a king on a hex with any number of stones of the opposite colour except the center hex."),
                        text("On your turn you may not:"),
                        text("Play on a hex that has been played on in the last turn."),
                )
                .width(Length::Fixed(300.0))
                .spacing(10)
                .align_items(Alignment::Center),
                button("Start").on_press(Message::Start),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::Center)
            .padding(Padding::from(100))
            .spacing(20)
            .into(),

            SixesUI::Running(state) => {
                let title_bar = match state.game.game_state() {
                    GameState::Win(player) => {
                        row![
                            text(format!("Player {:?} wins!", player)).size(50),
                            button("Menu").on_press(Message::Menu),
                        ]
                    }
                    GameState::Running => {
                        row![
                            text("Sixes").size(50),
                            button("Menu").on_press(Message::Menu),
                        ]
                    }
                };
                let stone_amt_pl1 = text(format!("Stones: {}", state.player_one.stones()));
                let stone_amt_pl2 = text(format!("Stones: {}", state.player_two.stones()));
                let player_one_control = row![
                    text("Player One"),
                    if state.player_one.stones() == 0
                        || !(state.game.turn() == PlayerID::One)
                        || state.game.game_state() != GameState::Running
                    {
                        button(stone_amt_pl1)
                    } else {
                        button(stone_amt_pl1).on_press(Message::SelectStone)
                    },
                    if state.player_one.king()
                        && state.game.turn() == PlayerID::One
                        && state.game.game_state() == GameState::Running
                    {
                        button("King").on_press(Message::SelectKing)
                    } else {
                        button("King")
                    },
                ];
                let board = Canvas::new(HexagonGridCanvas {
                    grid: HexagonGrid::new(5.0, 5.0),
                    state: state.clone(),
                })
                .width(Length::Fixed(800.0))
                .height(Length::Fill);
                let player_two_control = row![
                    text("Player Two"),
                    if state.player_two.stones() == 0
                        || !(state.game.turn() == PlayerID::Two)
                        || state.game.game_state() != GameState::Running
                    {
                        button(stone_amt_pl2)
                    } else {
                        button(stone_amt_pl2).on_press(Message::SelectStone)
                    },
                    if state.player_two.king()
                        && state.game.turn() == PlayerID::Two
                        && state.game.game_state() == GameState::Running
                    {
                        button("King").on_press(Message::SelectKing)
                    } else {
                        button("King")
                    },
                ];

                column![title_bar, player_one_control, board, player_two_control,]
                    .width(Length::Fill)
                    .align_items(Alignment::Center)
                    .into()
            }
        }
    }
}

#[derive(Debug, Clone)]
struct HexagonGridCanvas {
    grid: HexagonGrid,
    state: State,
}

//drawing hexagon grids
impl Program<Message> for HexagonGridCanvas {
    type State = ();
    fn update(
        &self,
        _state: &mut Self::State,
        event: Event,
        bounds: iced::Rectangle,
        cursor: Cursor,
    ) -> (event::Status, Option<Message>) {
        let cursor_position = if let Some(position) = cursor.position_in(&bounds) {
            position
        } else {
            return (event::Status::Ignored, None);
        };
        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::ButtonPressed(mouse::Button::Left) => {
                    (event::Status::Captured, Some(Message::SelectHex))
                }
                mouse::Event::CursorMoved { .. } => {
                    let coord = self.grid.in_hexagon(cursor_position, bounds.width);
                    if let Some(coord) = coord {
                        (event::Status::Captured, Some(Message::OverHex(Some(coord))))
                    } else {
                        (event::Status::Captured, Some(Message::OverHex(None)))
                    }
                }
                _ => (event::Status::Ignored, None),
            },
            _ => (event::Status::Ignored, None),
        }
    }

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &theme::Theme,
        bounds: iced::Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let hex_frame = self.grid.draw_hexes(bounds);

        let mut circle_frame: Frame = Frame::new(bounds.size());

        let circles = self
            .grid
            .draw_circles(bounds.width, self.state.hexes_selectable.clone());
        for circle in circles {
            circle_frame.stroke(
                &circle,
                Stroke::default()
                    .with_width(2.0)
                    .with_color(Color::from_rgb(1.0, 0.0, 0.0)),
            );
        }

        let piece_frame = self.grid.draw_pieces(
            bounds,
            self.state.game.board(),
            self.state.player_one,
            self.state.player_two,
        );

        vec![
            hex_frame.into_geometry(),
            circle_frame.into_geometry(),
            piece_frame.into_geometry(),
        ]
    }
}
