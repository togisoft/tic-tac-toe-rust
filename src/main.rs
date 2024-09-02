use std::io::stdin;
use colored::Colorize;

fn draw_board(board: [[char; 3]; 3]) {
    println!("{}", "----------".yellow());
    for i in 0..3 {
        print!("{}", "|".yellow());
        for j in 0..3 {
            let symbol = match board[i][j] {
                'X' => "X".bright_red().to_string(),
                'O' => "O".bright_green().to_string(),
                _ => " ".to_string(),
            };
            print!("{} {}", symbol, "|".yellow());
        }
        println!("{}", "\n----------".yellow());
    }
}

fn check_win(board: [[char; 3]; 3], player: char) -> bool {
    for i in 0..3 {
        if board[i][0] == player && board[i][1] == player && board[i][2] == player {
            return true;
        }
        if board[0][i] == player && board[1][i] == player && board[2][i] == player {
            return true;
        }
    }

    if board[0][0] == player && board[1][1] == player && board[2][2] == player {
        return true;
    }
    if board[0][2] == player && board[1][1] == player && board[2][0] == player {
        return true;
    }

    return false;
}

fn main() {
    let mut board = [[' '; 3]; 3];
    let mut player: char = 'X';
    println!("{}", "Welcome to Tic-Tac-Toe!");

    for _turn in 0..9 {
        draw_board(board);

        loop {
            println!("Player {}, enter row (0 - 2): ", player.to_string().bright_blue());
            let mut row = String::new();
            stdin().read_line(&mut row).expect("Row read error!");

            println!("Player {}, enter column (0 - 2): ", player.to_string().bright_blue());
            let mut col = String::new();
            stdin().read_line(&mut col).expect("Col read error!");

            let row_i32: usize = match row.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("{}", "Input not an integer! Try again.".red());
                    continue;
                }
            };

            let col_i32: usize = match col.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("{}", "Input not an integer! Try again.".red());
                    continue;
                }
            };

            if row_i32 > 2 || col_i32 > 2 {
                println!(
                    "{}",
                    "Invalid move! Index out of bounds. Try again!".on_bright_purple()
                );
                continue;
            }

            if board[row_i32][col_i32] != ' ' {
                println!(
                    "{}",
                    "Invalid move! Spot already taken. Try again!".purple()
                );
            } else {
                board[row_i32][col_i32] = player;
                break;
            }
        }

        if check_win(board, player) {
            draw_board(board);
            println!(
                "{} {} {}",
                "Player".yellow(),
                player.to_string().bright_blue(),
                "wins!".yellow()
            );
            return;
        }

        player = if player == 'X' { 'O' } else { 'X' };
    }

    println!("It's a draw!");
}
