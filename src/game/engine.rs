#[derive(Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn opponent(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

pub enum GameState {
    InProgress,
    Win(Player),
    Draw,
}

pub struct Game {
    pub board: [[Option<Player>; 3]; 3],
    pub current_player: Player,
    pub state: GameState,
    pub scores: (usize, usize),
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board: [[None; 3]; 3],
            current_player: Player::X,
            state: GameState::InProgress,
            scores: (0, 0),
        }
    }
}

impl Game {
    pub fn make_move(&mut self, row: usize, col: usize) {
        if let GameState::InProgress = self.state {
            if self.board[row][col].is_none() {
                self.board[row][col] = Some(self.current_player);
                if self.check_win() {
                    self.state = GameState::Win(self.current_player);
                    match self.current_player {
                        Player::X => self.scores.0 += 1,
                        Player::O => self.scores.1 += 1,
                    }
                } else if self.is_board_full() {
                    self.state = GameState::Draw;
                } else {
                    self.current_player = self.current_player.opponent();
                }
            }
        }
    }

    fn check_win(&self) -> bool {
        let p = Some(self.current_player);
        // Check rows
        for row in 0..3 {
            if self.board[row][0] == p && self.board[row][1] == p && self.board[row][2] == p {
                return true;
            }
        }

        // Check columns
        for col in 0..3 {
            if self.board[0][col] == p && self.board[1][col] == p && self.board[2][col] == p {
                return true;
            }
        }

        // Check diagonals
        if (self.board[0][0] == p && self.board[1][1] == p && self.board[2][2] == p)
            || (self.board[0][2] == p && self.board[1][1] == p && self.board[2][0] == p)
        {
            return true;
        }

        false
    }

    fn is_board_full(&self) -> bool {
        self.board
            .iter()
            .all(|row| row.iter().all(|cell| cell.is_some()))
    }

    pub fn restart(&mut self) {
        self.board = [[None; 3]; 3];
        self.current_player = Player::X;
        self.state = GameState::InProgress;
    }
}
