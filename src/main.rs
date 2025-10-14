use std::io;

fn main() {
    let mut board: Vec<Vec<char>> = vec![vec![' '; 3]; 3];
    let mut current_player: char = 'O';

    loop {
        println!("======================================");
        draw_board(&board);
        play_turn(&mut board, current_player);
        if check_over(&mut board, current_player) {
            break;
        }
        current_player = if current_player == 'X' { 'O' } else { 'X' };
    }
}

fn check_over(board: &mut Vec<Vec<char>>, player: char) -> bool {
    let win = (0..3).any(|i| {
        (board[i][0] == player && board[i][1] == player && board[i][2] == player)
            || (board[0][i] == player && board[1][i] == player && board[2][i] == player)
            || (board[0][0] == player && board[1][1] == player && board[2][2] == player
                || board[0][2] == player && board[1][1] == player && board[2][0] == player)
    });
    if win {
        println!("Player {player} won!");
        return true;
    }
    if board.iter().all(|r| r.iter().all(|&c| c != ' ')) {
        println!("It's a draw!");
        return true;
    }
    false
}

fn play_turn(board: &mut Vec<Vec<char>>, player: char) {
    loop {
        let pos = take_player_input(player);
        let row = (pos - 1) / 3;
        let col = (pos - 1) % 3;

        if board[row as usize][col as usize] == ' ' {
            board[row as usize][col as usize] = player;
            break;
        } else {
            println!("Spot is taken Choose another");
            continue;
        }
    }
}

fn take_player_input(current_player: char) -> u32 {
    loop {
        let mut input = String::new();
        println!("Player {current_player} enter your position: ");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<u32>() {
            Ok(num) if num >= 1 && num <= 9 => return num,
            _ => {
                println!("Invalid input. please enter number between 1 and 9");
                continue;
            }
        }
    }
}

fn draw_board(board: &Vec<Vec<char>>) {
    println!();
    for (i, row) in board.iter().enumerate() {
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
