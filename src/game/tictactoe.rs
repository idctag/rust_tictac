use crate::game::engine::{Game, GameState};
use egui::{Button, CentralPanel, Grid};

pub struct App {
    game: Game,
}

impl Default for App {
    fn default() -> Self {
        Self {
            game: Game::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Tic Tac Toe");
                ui.separator();

                let message = match self.game.state {
                    GameState::InProgress => format!("Player {}'s turn", self.game.current_player),
                    GameState::Win(player) => format!("Player {} wins!", player),
                    GameState::Draw => "It's a Draw!".to_string(),
                };
                ui.label(&message);

                ui.separator();

                ui.horizontal(|ui| {
                    ui.add_space((ui.available_width() - 190.0) / 2.0);
                    Grid::new("tic_tac_board")
                        .spacing([20.0, 20.0])
                        .max_col_width(50.0)
                        .show(ui, |ui| {
                            for row in 0..3 {
                                for col in 0..3 {
                                    let cell = self.game.board[row][col];
                                    let label = cell.map_or(" ".to_string(), |p| p.to_string());
                                    if ui
                                        .add_sized([50.0, 50.0], Button::new(label))
                                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                                        .clicked()
                                    {
                                        self.game.make_move(row, col);
                                    }
                                }
                                ui.end_row();
                            }
                        });
                });
                ui.separator();

                ui.label(format!(
                    "Scores: X = {}, O = {}",
                    self.game.scores.0, self.game.scores.1
                ));

                if !matches!(self.game.state, GameState::InProgress) {
                    ui.horizontal(|ui| {
                        if ui.button("Restart").clicked() {
                            self.game.restart();
                        }
                        if ui.button("Quit").clicked() {
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                }
            });
        });
    }
}
