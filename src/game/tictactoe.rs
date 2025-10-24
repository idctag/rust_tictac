pub mod tic_tac {
    use std::io;

    struct Player {
        name: char,
        score: usize,
    }

    pub struct App {
        board: [[char; 3]; 3],
        current_player_index: usize,
        game_over: bool,
        winner: Option<char>,
        message: String,
        quit: bool,
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
                quit: false,
            }
        }
    }

    impl App {
        fn get_current_player(&self) -> &Player {
            if self.current_player_index == 0 {
                &self.player_one
            } else {
                &self.player_two
            }
        }

        fn get_current_player_mut(&mut self) -> &mut Player {
            if self.current_player_index == 0 {
                &mut self.player_one
            } else {
                &mut self.player_two
            }
        }

        fn switch_player(&mut self) {
            self.current_player_index = if self.current_player_index == 0 { 1 } else { 0 }
        }

        fn reset(&mut self) {
            self.board = [[' '; 3]; 3];
            self.current_player_index = 0;
            self.game_over = false;
            self.winner = None;
            self.message = String::from("Player X's turn");
        }

        fn make_move(&mut self, row: usize, col: usize) {
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
                self.ask_continue();
            } else if self.is_board_full() {
                self.game_over = true;
                self.winner = None;
                self.message = format!("It's a Draw!");
                self.ask_continue();
            } else {
                self.switch_player();
                let newgame = self.get_current_player().name;
                self.message = format!("Player {}'s turn", newgame)
            }
        }

        fn ask_continue(&mut self) {
            println!("Play again? [y/n]");
            loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let trimmed = input.trim().to_lowercase();

                if let Some(c) = trimmed.chars().next() {
                    match c {
                        'n' => {
                            self.quit = true;
                            self.display_scores();
                            break;
                        }
                        'y' => {
                            self.reset();
                            break;
                        }
                        _ => println!("Please enter 'y' or 'n'"),
                    }
                } else {
                    println!("Please enter 'y' or 'n'")
                }
            }
        }

        fn check_win(&self, player: char) -> bool {
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

        fn take_input(&self) -> (usize, usize) {
            loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                match input.trim().parse::<i32>() {
                    Ok(num) if num >= 1 && num <= 9 => {
                        return (((num - 1) / 3) as usize, ((num - 1) % 3) as usize);
                    }
                    _ => println!("Please enter number between 1 and 9"),
                }
            }
        }

        fn draw_board(&self) {
            println!();
            for (i, row) in self.board.iter().enumerate() {
                let display_row: Vec<String> = row
                    .iter()
                    .enumerate()
                    .map(|(j, &cell)| {
                        if cell == ' ' {
                            (i * 3 + j + 1).to_string()
                        } else {
                            cell.to_string()
                        }
                    })
                    .collect();
                println!(
                    " {} | {} | {}",
                    display_row[0], display_row[1], display_row[2]
                );
                if i < 2 {
                    println!("---------")
                }
            }
        }

        fn display_scores(&self) {
            println!("╔════════════════════════════╗");
            println!("║         SCORES             ║");
            println!("╠════════════════════════════╣");
            println!(
                "║ Player {} : {:2}             ║",
                self.player_one.name, self.player_one.score
            );
            println!(
                "║ Player {} : {:2}             ║",
                self.player_two.name, self.player_two.score
            );
            println!("╚════════════════════════════╝");
        }
        pub fn play(&mut self) {
            println!("{}", self.message);
            while !self.quit {
                self.draw_board();
                let (row, col) = self.take_input();
                self.make_move(row, col);
                println!("{}", self.message)
            }
        }
    }
}
