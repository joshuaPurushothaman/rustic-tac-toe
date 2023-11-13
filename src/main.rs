use personal_tic_tac_toe::board::*;
use rand::seq::SliceRandom;
use std::io::{self, Write};

// TODO: use piston instead :3
fn main() {
    let mut board = Board::new();

    let stdin = io::stdin();

    println!(
        "Welcome to Tic-Tac-Toe!
    
    You play as X.
    Ctrl-C to exit at any point.
    If you see the same prompt twice, re-enter your input correctly.
    
    Good luck!"
    );

    print!("Who would you like to play as? X or O (default X, who plays first): ");
    io::stdout().flush().unwrap(); //  Flush stdio so it prints in correct order!
    let mut user_player_input = String::new();
    stdin.read_line(&mut user_player_input).unwrap();
    let user_player = match user_player_input.trim().to_ascii_lowercase().as_str() {
        "x" => Player::X,
        "o" => Player::O,
        _ => Player::X,
    };
    let computer_player = match user_player {
        Player::X => Player::O,
        Player::O => Player::X,
    };

    // X has to play first...
    if computer_player == Player::X {
        // Instead of minimaxing the very first move, just hardcode center and corners as "decent" first moves
        let good_first_moves = vec![(1, 1), (0, 0), (2, 0), (0, 2), (2, 2)];

        let computer_move = good_first_moves
            .choose(&mut rand::thread_rng())
            .expect("computer fail moment");

        board
            .set_cell(computer_move.0, computer_move.1, computer_player)
            .unwrap();
    }

    print!("\x1B[2J\x1B[1;1H"); // magical clearscreen :3

    let has_won = false;
    while !has_won {
        println!("{board}");

        print!("What's your move? (Indexed at 0, formatted 'x y'): ");
        io::stdout().flush().unwrap();
        let mut user_input = String::new();
        stdin.read_line(&mut user_input).unwrap();

        // rust strings are weird bc of UTF-8!
        // as_bytes() here assumes user entered ASCII...
        let x = match (user_input.as_bytes()[0] as char).to_digit(10) {
            Some(val) => match val {
                0..=2 => val as i32,
                _ => continue,
            },
            None => continue,
        };
        // TODO: i will never find a tidy way for this, lol.
        // Maybe a "more proper" state mgmt system, taking the loop out?
        // That way, we could use the ? operator to propagate errors...
        // !!! edit: I MAY HAVE FOUND IT: `if let` is useful here!
        // https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
        let y = match (user_input.as_bytes()[2] as char).to_digit(10) {
            Some(val) => match val {
                0..=2 => val as i32,
                _ => continue, // TODO: could care a tiny bit more and tell the user. lol
            },
            None => continue,
        };

        if board.get_cell(x, y) == None {
            // Sort of okay to unwrap here for now... the "coord in range" case was handled already.
            board.set_cell(x, y, user_player).unwrap();
        } else {
            continue;
        }

        // Check if the player's won by now.
        if board.check_winner() == Some(user_player) {
            println!("{board}");
            println!("\n\nYou win as {user_player}!\n\n");
            break;
        }

        // Computer now makes a move!
        let available_cells = board.get_available_cells();

        // TODO MINIMAX!!!
        let computer_move = available_cells
            .choose(&mut rand::thread_rng())
            .expect("computer fail moment");

        board
            .set_cell(computer_move.0, computer_move.1, computer_player)
            .unwrap();

        print!("\x1B[2J\x1B[1;1H"); // magical clearscreen :3

        if board.check_winner() == Some(computer_player) {
            println!("{board}");
            println!("\n\nComputer wins as {computer_player}!\n\n");
            break;
        }
    }
}
