use eframe::epaint::{CircleShape, RectShape};
use egui::{
    Color32, Context, PointerButton, Pos2, Rect, Rounding, Shape, SidePanel, Stroke, TextEdit,
    TextStyle, Vec2, Window,
};

use crate::game::{Grid, Player, Turn, Variant};

const BOX_SIZE: f32 = 60.0;
const GRID_SIZE: f32 = 3.0 * BOX_SIZE;

#[derive(Default)]
struct UiState {
    notation_textbox_content: String,
}

pub struct App {
    board: Grid,
    state: UiState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            board: Grid::default().with_variant(Variant::Relative),
            state: UiState::default(),
        }
    }
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        SidePanel::left("options").show(ctx, |ui| {
            ui.heading("Options");
            // [] Play full random game (ignores following options, possibly has some config)
            // [] Start game with random moves played (# moves)
            // [] Show what squares opponent will be able to use
            // Mate in N finder (if I can make it)
        });

        SidePanel::right("notation").show(ctx, |ui| {
            ui.heading("Game");
            ui.add(
                TextEdit::multiline(&mut self.state.notation_textbox_content)
                    .font(TextStyle::Monospace),
            );
            if ui.button("Replay game from notation").clicked()
                && !self.state.notation_textbox_content.is_empty()
            {
                self.board = Grid::default();
                for turn in self.state.notation_textbox_content.split('\n') {
                    let turn: Turn = turn.try_into().unwrap();
                    self.board.apply_turn(turn.coords).unwrap(); // XXX: REMOVE UNWRAPS
                }
            }
            // New Game
            // <Online stuff?>
            // Current game notation
            // Way to traverse moves -> (with fancy tree?)
        });

        Window::new("Board")
            .fixed_size(Vec2::splat(GRID_SIZE * 3.0))
            .show(ctx, |ui| {
                let painter = ui.painter();

                let origin = ui.min_rect().min;

                let mut squares = Vec::new();
                let mut pieces = Vec::new();

                let (pos, interact_pos) = {
                    let pointer = &ui.input().pointer;
                    (
                        pointer.hover_pos(),
                        if pointer.button_clicked(PointerButton::Primary) {
                            pointer.interact_pos()
                        } else {
                            None
                        },
                    )
                };

                let valid_boxes = self.board.get_valid_boxes(self.board.get_track());

                for ix in 0..3 {
                    for iy in 0..3 {
                        let outer_coords = (ix, iy).try_into().unwrap();
                        for jx in 0..3 {
                            for jy in 0..3 {
                                let inner_coords = (jx, jy).try_into().unwrap();

                                let rect = Rect::from_min_size(
                                    Pos2::new(
                                        origin.x + ix as f32 * GRID_SIZE + jx as f32 * BOX_SIZE,
                                        origin.y + iy as f32 * GRID_SIZE + jy as f32 * BOX_SIZE,
                                    ),
                                    Vec2::splat(BOX_SIZE),
                                );

                                let color = if valid_boxes.contains(&(outer_coords, inner_coords)) {
                                    Color32::from_rgba_unmultiplied(10, 255, 100, 40)
                                } else {
                                    Color32::TRANSPARENT
                                };

                                if let Some(pos) = interact_pos {
                                    if rect.contains(pos) {
                                        if let Ok(_) =
                                            self.board.apply_turn((outer_coords, inner_coords))
                                        {
                                            self.state.notation_textbox_content =
                                                self.board.to_string();
                                        }
                                    }
                                }

                                squares.push(
                                    RectShape {
                                        rect,
                                        rounding: Rounding::none(),
                                        fill: color,
                                        stroke: Stroke::new(0.5, Color32::WHITE),
                                    }
                                    .into(),
                                );

                                if let Some(pos) = pos {
                                    if rect.contains(pos) {
                                        squares.push(
                                            RectShape::filled(
                                                rect,
                                                Rounding::none(),
                                                Color32::from_rgba_unmultiplied(255, 255, 255, 15),
                                            )
                                            .into(),
                                        );
                                    }
                                }

                                if let Some(player) =
                                    self.board.get_box(outer_coords).get_tile(inner_coords)
                                {
                                    match *player {
                                        Player::X => {
                                            pieces.push(Shape::line_segment(
                                                [rect.left_bottom(), rect.right_top()],
                                                (3.0, Color32::RED),
                                            ));
                                            pieces.push(Shape::line_segment(
                                                [rect.left_top(), rect.right_bottom()],
                                                (3.0, Color32::RED),
                                            ));
                                        }
                                        Player::O => {
                                            pieces.push(
                                                CircleShape {
                                                    center: rect.center(),
                                                    radius: (BOX_SIZE - 5.0) / 2.0,
                                                    fill: Color32::TRANSPARENT,
                                                    stroke: (3.0, Color32::BLUE).into(),
                                                }
                                                .into(),
                                            );
                                        }
                                    }
                                }
                            }
                        }
                        squares.push(
                            RectShape {
                                rect: Rect::from_min_size(
                                    Pos2::new(
                                        origin.x + ix as f32 * GRID_SIZE,
                                        origin.y + iy as f32 * GRID_SIZE,
                                    ),
                                    Vec2::splat(GRID_SIZE),
                                ),
                                rounding: Rounding::none(),
                                fill: if let Some(player) = self.board.get_box(outer_coords).winner
                                {
                                    match player {
                                        Player::X => Color32::from_rgba_unmultiplied(255, 0, 0, 20),
                                        Player::O => Color32::from_rgba_unmultiplied(0, 0, 255, 20),
                                    }
                                } else {
                                    Color32::TRANSPARENT
                                },
                                stroke: Stroke::new(1.0, Color32::WHITE),
                            }
                            .into(),
                        );
                    }
                }
                painter.extend(pieces);
                painter.extend(squares);
            });
    }
}
