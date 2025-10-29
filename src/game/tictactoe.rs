pub mod tic_tac {
    use egui::{Button, CentralPanel, Grid};

    pub struct Player {
        name: char,
        score: usize,
    }

    impl eframe::App for App {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Tic tac toe");
                    ui.label(&self.message);
                    ui.separator();
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        Grid::new("tic_tac_board")
                            .spacing([20.0, 20.0])
                            .max_col_width(50.0)
                            .show(ui, |ui| {
                                for row in 0..3 {
                                    for col in 0..3 {
                                        let cell = self.board[row][col];
                                        let label =
                                            if cell == ' ' { " " } else { &cell.to_string() };
                                        if ui.add_sized([50.0, 50.0], Button::new(label)).clicked()
                                        {
                                            self.make_move(row, col);
                                        }
                                    }
                                    ui.end_row();
                                }
                            });
                    });
                    ui.separator();
                    ui.label(format!(
                        "Scores: X = {}, O = {}",
                        self.player_one.score, self.player_two.score
                    ));
                    if self.game_over {
                        if ui.button("Restar").clicked() {
                            *self = App::default()
                        }
                    }
                });
            });
        }
    }

    pub struct App {
        board: [[char; 3]; 3],
        current_player_index: usize,
        game_over: bool,
        winner: Option<char>,
        message: String,
        player_one: Player,
        player_two: Player,
    }

    impl Default for App {
        fn default() -> Self {
            let player_one = Player {
                name: 'X',
                score: 0,
            };
            let player_two = Player {
                name: 'O',
                score: 0,
            };
            Self {
                board: [[' '; 3]; 3],
                player_one,
                player_two,
                current_player_index: 0,
                game_over: false,
                winner: None,
                message: String::from("Player X's turn"),
            }
        }
    }

    impl App {
        pub fn get_current_player(&self) -> &Player {
            if self.current_player_index == 0 {
                &self.player_one
            } else {
                &self.player_two
            }
        }

        pub fn get_current_player_mut(&mut self) -> &mut Player {
            if self.current_player_index == 0 {
                &mut self.player_one
            } else {
                &mut self.player_two
            }
        }

        pub fn switch_player(&mut self) {
            self.current_player_index = if self.current_player_index == 0 { 1 } else { 0 }
        }

        pub fn make_move(&mut self, row: usize, col: usize) {
            if self.game_over || self.board[row][col] != ' ' {
                return;
            }

            let current_name = self.get_current_player().name;
            self.board[row][col] = current_name;

            if self.check_win(current_name) {
                self.game_over = true;
                self.winner = Some(self.get_current_player().name);
                self.get_current_player_mut().score += 1;
                self.message = format!("Player {} wins!", current_name);
            } else if self.is_board_full() {
                self.game_over = true;
                self.winner = None;
                self.message = format!("It's a Draw!");
            } else {
                self.switch_player();
                let newgame = self.get_current_player().name;
                self.message = format!("Player {}'s turn", newgame)
            }
        }

        pub fn check_win(&self, player: char) -> bool {
            for row in 0..3 {
                if self.board[row][0] == player
                    && self.board[row][1] == player
                    && self.board[row][2] == player
                {
                    return true;
                }
            }

            for col in 0..3 {
                if self.board[0][col] == player
                    && self.board[1][col] == player
                    && self.board[2][col] == player
                {
                    return true;
                }
            }

            if (self.board[0][0] == player
                && self.board[1][1] == player
                && self.board[2][2] == player)
                || (self.board[0][2] == player
                    && self.board[1][1] == player
                    && self.board[2][0] == player)
            {
                return true;
            }

            false
        }

        fn is_board_full(&self) -> bool {
            self.board.iter().all(|row| row.iter().all(|&c| c != ' '))
        }
    }
}
