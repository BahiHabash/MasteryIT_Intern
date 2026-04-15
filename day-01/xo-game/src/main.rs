use std::io;
use colored::*;

fn print_board(board: &[char; 9]) {
    println!("
            |---|---|---|
            | {} | {} | {} |
            |---|---|---|
            | {} | {} | {} |
            |---|---|---|
            | {} | {} | {} |
            |---|---|---|
        ", 
        board[0], board[1], board[2], 
        board[3], board[4], board[5], 
        board[6], board[7], board[8]
    );
}

fn check_win(board: &[char; 9]) -> bool {
    let wining_positions: [[usize; 3]; 8] = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8],
        [0, 3, 6], [1, 4, 7], [2, 5, 8],
        [0, 4, 8], [2, 4, 6],
    ];

    for positions in wining_positions {
        if board[positions[0]] == board[positions[1]] && board[positions[1]] == board[positions[2]] {
            return true;
        }
    }   

    return false;
}   

fn check_draw(board: &[char; 9]) -> bool {
    return board.iter().all(|&pos| pos == 'O' || pos == 'X');
}

fn main() {
    let mut board: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let mut curr_player: char = 'X';

    println!("{}", "Welcome to tic-tac-toe game!".cyan().bold());
    
    loop {
        print_board(&board);
        println!("{}", format!("Player {}, pick a number to replace.", curr_player.to_string().bold()).cyan());    
        let mut input: String = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect(&"Please enter a valid number".red().to_string());

        let index: usize = match input.trim().parse::<usize>() {
            Ok(num) if (1..=9).contains(&num) => num - 1,
            _ => {
                println!("{}", "Invalid input. Please enter a valid number between 1 and 9.".red());
                continue;
            }
        };

        // Check if the chosen spot is already taken
        if board[index] == 'X' || board[index] == 'O' {
            println!("{}", "That spot is already taken! Try a different one.".red());
            continue;
        }

        board[index] = curr_player;

        if check_win(&board) {
            println!("{}", format!("Player {}, won the game 🎉!", curr_player.to_string().bold()).green());            
            print_board(&board);
            break;
        } else if check_draw(&board) {
            println!("{}", "Game is draw 😑!".yellow().bold());
            print_board(&board);
            break;
        }
    
        curr_player = if curr_player == 'X' {'O'} else {'X'};
    }
}
