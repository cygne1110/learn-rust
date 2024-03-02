use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

const EMPTY: char = ' ';
const PLUS_INF: f64 = 9999.0;
const MINUS_INF: f64 = -9999.0;
const WIN: f64 = 10.0;
const LOSE: f64 = -1.0;
const PRINT_PATH: bool = false;

fn main() {

    let dataset_path: String = String::from("data/dataset.txt");
    
    // Get board lines
    if let Ok(lines) = read_lines(&dataset_path) {

        let mut sum = 0.0;
        let mut time_sum = 0;
        let mut iteration = 0;

        // Consumes the iterator, returns an (Optional) String
        for mut line in lines.flatten() {

            if PRINT_PATH {
                println!("======= ITERATION {} ========", iteration);
            }
            
            // Get board from file (kinda obscure tbh)
            line.truncate(10);
            let mut chars = line.chars();
            let curr_player: char = chars.next().unwrap();
            let board: [char; 9];
            board = to_array::<9>(line.get(1..).unwrap());

            // Getting the token form who needs to win
            let winner = curr_player;
            let loser;
            if curr_player == 'X' {
                loser = 'O';
            } else {
                loser = 'X';
            }

            // Time measurement
            let now = Instant::now();
            sum += minimax(&board, curr_player, 0, &winner, &loser);
            let elapsed = now.elapsed();
            time_sum += elapsed.as_millis();

            if PRINT_PATH {
                retrace_path(&board, curr_player);
            }
            
            iteration = iteration + 1;

        }

        println!("Checked {} board states in {}ms", iteration, time_sum);
        println!("Number check: {}", sum);

    } else {
        println!("Could not open file {}!", dataset_path);
    }

}

// MiniMax
fn minimax(board: &[char; 9], curr_player: char, depth: i32, winner: &char, loser: &char) -> f64 {

    let state = check_state(board, winner, loser);// 0 is TIE, 10 is WINNER, -1 is LOSER
    let childs = get_childs(board, curr_player);
    // Board is in final state
    if state > MINUS_INF {
        // Weighting the value of a leaf with the depth of research, to get faster wins
        state - if state == WIN {f64::from(depth)} else {0.0}

    } else if curr_player == *winner {

        let mut m = MINUS_INF;
        for c in childs {
            m = f64::max(m, minimax(&c, *loser, depth + 1, winner, loser));
        }
        m

    } else {

        let mut m = PLUS_INF;
        for c in childs {
            m = f64::min(m, minimax(&c, *winner, depth + 1, winner, loser));
        }
        m

    }

}

// Prints the path to VICTORY (or TIE D:) from an initial board using minimax
fn retrace_path(board: &[char; 9], curr_player: char) {
    
    let winner = curr_player;
    let loser;
    if curr_player == 'X' {
        loser = 'O';
    } else {
        loser = 'X';
    }

    // A finished board shouldn't have childs
    if check_state(&board, &winner, &loser) > MINUS_INF {
        return
    }

    let childs = get_childs(&board, curr_player);

    // Gathering scores
    let mut score: Vec<f64> = Vec::new();
    for i in 0..childs.len() {
        score.push(minimax(&childs[i], loser, 1, &winner, &loser));
    }

    let mut choice = 0;
    let mut max = MINUS_INF;

    println!("{:?}", score);
    // Getting best child index
    for i in 0..score.len() {
        if score[i] > max {
            max = score[i];
            choice = i;
        }
    }

    let new_board = childs[choice].clone();

    print_board(&board, curr_player);
    print_board(&new_board, loser);

    println!("");

    // We go again
    retrace_path(&new_board, loser);

}

// Prints the board !!!!!!
fn print_board(board: &[char; 9], curr_player: char) {

    println!("Turn: {}", curr_player);
    for i in 0..3 {
        println!("{}{}{}", board[i*3], board[i*3+1], board[i*3+2]);
    }

}

// Return all possible child
fn get_childs(board: &[char; 9], curr_player: char) -> Vec<[char; 9]> {

    let mut v: Vec<[char; 9]> = Vec::new();
    for i in 0..9 {
        if board[i] == EMPTY {
            let mut child: [char; 9] = board.clone();
            child[i] = curr_player;
            v.push(child);
        }
    }
    v

}

// Eldritch horror
// I am so sorry
fn check_state(board: &[char; 9], winner: &char, loser: &char) -> f64 {

    // FIRST PLAYER
    // first row
    if board[0] == *winner && board[0] == board[1] && board[1] == board[2] {
        return WIN
    }
    // second row
    if board[3] == *winner && board[3] == board[4] && board[4] == board[5] {
        return WIN
    }
    // third row
    if board[6] == *winner && board[6] == board[7] && board[7] == board[8] {
        return WIN
    }
    // first column
    if board[0] == *winner && board[0] == board[3] && board[3] == board[6] {
        return WIN
    }
    // second column
    if board[1] == *winner && board[1] == board[4] && board[4] == board[7] {
        return WIN
    }
    // third column
    if board[2] == *winner && board[2] == board[5] && board[5] == board[8] {
        return WIN
    }
    // up left to down right diag
    if board[0] == *winner && board[0] == board[4] && board[4] == board[8] {
        return WIN
    }
    // down left to up right diag
    if board[6] == *winner && board[6] == board[4] && board[4] == board[2] {
        return WIN
    }

    // SECOND PLAYER
    // first row
    if board[0] == *loser && board[0] == board[1] && board[1] == board[2] {
        return LOSE
    }
    // second row
    if board[3] == *loser && board[3] == board[4] && board[4] == board[5] {
        return LOSE
    }
    // third row
    if board[6] == *loser && board[6] == board[7] && board[7] == board[8] {
        return LOSE
    }
    // first column
    if board[0] == *loser && board[0] == board[3] && board[3] == board[6] {
        return LOSE
    }
    // second column
    if board[1] == *loser && board[1] == board[4] && board[4] == board[7] {
        return LOSE
    }
    // third column
    if board[2] == *loser && board[2] == board[5] && board[5] == board[8] {
        return LOSE
    }
    // up left to down right diag
    if board[0] == *loser && board[0] == board[4] && board[4] == board[8] {
        return LOSE
    }
    // down left to up right diag
    if board[6] == *loser && board[6] == board[4] && board[4] == board[2] {
        return LOSE
    }
        
    // check for TIE
    for i in 0..9 {
        if board[i] == EMPTY {
            return MINUS_INF;
        }
    }
    0.0
}

// Function from the Rust Programming Language Book
// - The output is wrapped in a Result to allow matching on errors.
// - Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Function from someone on Reddit lol
fn to_array<const N: usize>(s: &str) -> [char; N] {
    let mut chars = s.chars();
    [(); N].map(|_| chars.next().unwrap())
}
