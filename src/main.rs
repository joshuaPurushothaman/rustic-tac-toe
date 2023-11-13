use rand::seq::SliceRandom;
use rustic_tac_toe::board::*;
use std::io::{self, Write};

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
        let good_first_moves = [(1, 1), (0, 0), (2, 0), (0, 2), (2, 2)];

        let computer_move = good_first_moves.choose(&mut rand::thread_rng()).unwrap();

        board
            .set_cell(computer_move.0, computer_move.1, computer_player)
            .unwrap();
    }

    clear_screen();

    loop {
        println!("{board}"); // TODO: add colors in X/O output

        print!("What's your move? (Indexed at 0, formatted 'x y'): ");
        io::stdout().flush().unwrap();
        let mut user_input = String::new();
        stdin.read_line(&mut user_input).unwrap();

        // rust strings are weird bc of UTF-8!
        // as_bytes() here assumes user entered ASCII...
        let x = match char_to_coord(user_input.as_bytes()[0] as char) {
            Some(value) => value,
            None => continue,
        };
        let y = match char_to_coord(user_input.as_bytes()[2] as char) {
            Some(value) => value,
            None => continue,
        };

        if board.get_cell(x, y).is_none() {
            // Sort of okay to unwrap here for now... the "coord in range" case was handled already.
            board.set_cell(x, y, user_player).unwrap();
        } else {
            continue;
        }

        // Check if the player's won by now.
        if board.check_winner() == GameFinaleState::Win(user_player) {
            println!("{board}");
            println!("\n\nYou win as {user_player}!\n\n");
            break;
        }
        
        if board.check_winner() == GameFinaleState::Draw {
            println!("{board}");
            println!("\n\nDraw!\n\n");
            break;
        }

        let computer_move = minimax(&board, computer_player, (x, y)).coord.unwrap();

        board
            .set_cell(computer_move.0, computer_move.1, computer_player)
            .unwrap();

        clear_screen();

        if board.check_winner() == GameFinaleState::Win(computer_player) {
            println!("{board}");
            println!("\n\nComputer wins as {computer_player}!\n\n");
            break;
        } else if board.check_winner() == GameFinaleState::Draw {
            println!("{board}");
            println!("\n\nDraw!\n\n");
            break;
        }
    }
}
fn char_to_coord(user_input: char) -> Option<i32> {
    match user_input.to_digit(10) {
        Some(val) => match val {
            0..=2 => Some(val as i32),
            _ => None,
        },
        None => None,
    }
}

fn clear_screen() {
    // magical clearscreen :3
    // print!("\x1B[2J\x1B[1;1H");
}

#[derive(Clone, Copy)]
struct ScoredMove {
    coord: Option<(i32, i32)>,
    score: i32,
}

// TODO: figure out what the heck is happening here, and if it is working, then WHY
fn minimax(board: &Board, player: Player, uhh_coord: (i32, i32)) -> ScoredMove {
    if board.check_winner() != GameFinaleState::StillGoing {
        return ScoredMove {
            coord: Some(uhh_coord),
            score: score(*board, player),
        };
    }

    let opponent = match player {
        Player::X => Player::O,
        Player::O => Player::X,
    };

    let potential_moves = board.get_available_cells();
    let mut scored_moves: Vec<ScoredMove> = vec![];

    // testing...
    if potential_moves.is_empty() && board.check_winner() == GameFinaleState::StillGoing {
        panic!("AHHH");
    }

    for potential_move in potential_moves {
        let mut hypothetical_board = *board; // rely on Copy trait :)

        hypothetical_board
            .set_cell(potential_move.0, potential_move.1, player)
            .unwrap();

        let value = minimax(&hypothetical_board, opponent, potential_move);
        scored_moves.push(value);
    }

    if board.get_active_turn().unwrap() == player {
        *scored_moves.iter().min_by_key(|sm| sm.score).unwrap()
    } else {
        *scored_moves.iter().min_by_key(|sm| sm.score).unwrap()
    }
}

/// Remember, `score()` is called on end states of the game.
fn score(board: Board, player: Player) -> i32 {
    let win_score = 1;
    let draw_score = 0;
    let lose_score = -1;

    let winner = board.check_winner();
    let opponent = match player {
        Player::X => Player::O,
        Player::O => Player::X,
    };

    if winner == GameFinaleState::Win(player) {
        // print!("W"); // FIXME: !!! Why isn't it seeing any wins?!
        win_score
    } else if winner == GameFinaleState::Draw {
        // print!("D"); // TODO: use dbg!() and inspect https://dhghomon.github.io/easy_rust/Chapter_38.html
        draw_score
    } else if winner == GameFinaleState::Win(opponent) {
        // print!("L");
        lose_score
    } else {
        panic!("score() was called in a StillGoing game. player: {player}, board: {board}");
    }
}
