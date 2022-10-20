// Tic Tac Toe game in Rust

use std::io;
use std::io::Write;

fn main() {
    let mut board: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut turn: char = 'X';
    let mut count: u8 = 0;

    loop {
        for i in 0..9 {
            print!("{} ", board[i]);
            if i == 2 || i == 5 || i == 8 {
                println!("");
            }
        }

        let mut input = String::new();

        loop {
            print!("Turn for {}. Move on which space? ", turn);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();

            let num: u8 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("That's not a number!");
                    continue;
                }
            };

            if num > 9 {
                println!("That's not on the board!");
                continue;
            }

            if board[(num - 1) as usize] == 'X' || board[(num - 1) as usize] == 'O' {
                println!("That space is already taken!");
                continue;
            }

            break;
        }

        board[(input.trim().parse::<u8>().unwrap() - 1) as usize] = turn;

        if turn == 'X' {
            turn = 'O';
        } else {
            turn = 'X';
        }

        count += 1;

        for i in 0..3 {
            if board[i * 3] == board[i * 3 + 1] && board[i * 3 + 1] == board[i * 3 + 2] {
                println!("{} wins!", board[i * 3]);
                return;
            }

            if board[i] == board[i + 3] && board[i + 3] == board[i + 6] {
                println!("{} wins!", board[i]);
                return;
            }
        }

        if board[0] == board[4] && board[4] == board[8] {
            println!("{} wins!", board[0]);
            return;
        }

        if board[2] == board[4] && board[4] == board[6] {
            println!("{} wins!", board[2]);
            return;
        }

        if count == 9 {
            println!("The game is a draw!");
            return;
        }
    }
}