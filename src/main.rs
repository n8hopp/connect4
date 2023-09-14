
use std::io::{stdin,stdout, Write, empty};
use std::error::Error;
use std::fmt::{Display,Formatter};
use std::fmt;

#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell};

/////////////////////////////////////////////
//
// Data structres used in Connect 4
//
/////////////////////////////////////////////

/// A Connect 4 piece
/// A piece can either be 
///  * R (player 1)
///  * B (player 2)
///  * E (empty)
#[derive(PartialEq, Clone, Copy)]
enum Piece { R, B, E }

use Piece::{R, B, E};

/// A Connect 4 board is a 2d vector of pieces
type Board = Vec<Vec<Piece>>;

// a move on the board, this is a pair of row and column
type Move = usize;

/////////////////////////////////////////////
//
// Code for printing out the board
//
/////////////////////////////////////////////

/// Prints out the Connect 4 board
/// The format of the board should be
///
///    1 2 3 4 5 6 7
///   +-------------+
/// A | | | | | | | |
///   |-+-+-+-+-+-+-|
/// B | | | | | | | |
///   |-+-+-+-+-+-+-|
/// C | | | | | | | |
///   |-+-+-+-+-+-+-|
/// D | | | | | | | |
///   |-+-+-+-+-+-+-|
/// E | | | | | | | |
///   |-+-+-+-+-+-+-|
/// F | | | | | | | |
///   +-------------+
///
/// # Arguments
/// 
/// * board - The board to print out
fn print_board(board : &Board)
{
    // Part 1: print out the board.
    // It should be formatted like the comment above.
    let mut table = Table::new();

    table.add_row(row![" ", "1", "2", "3", "4", "5", "6", "7"]);
    table.add_row(row!["A", board[0][0], board[0][1], board[0][2], board[0][3], board[0][4], board[0][5], board[0][6]]);
    table.add_row(row!["B", board[1][0], board[1][1], board[1][2], board[1][3], board[1][4], board[1][5], board[1][6]]);
    table.add_row(row!["C", board[2][0], board[2][1], board[2][2], board[2][3], board[2][4], board[2][5], board[2][6]]);
    table.add_row(row!["D", board[3][0], board[3][1], board[3][2], board[3][3], board[3][4], board[3][5], board[3][6]]);
    table.add_row(row!["E", board[4][0], board[4][1], board[4][2], board[4][3], board[4][4], board[4][5], board[4][6]]);
    table.add_row(row!["F", board[5][0], board[5][1], board[5][2], board[5][3], board[5][4], board[5][5], board[5][6]]);

    table.printstd();

    println!("{}", board[5][0]);
}

/// create a new board that is a 7x6 vector with all empty squares.
fn new_board() -> Board
{
    vec![vec![E; 7]; 6]
}

/// creates a test board to test printing
///
/// print(test_board()) should print out
///    1 2 3 4 5 6 7
///   +-------------+
/// A | | | | | | | |
///   |-+-+-+-+-+-+-|
/// B | | | | | | | |
///   |-+-+-+-+-+-+-|
/// C |B|R|B|R|B|R|B|
///   |-+-+-+-+-+-+-|
/// D |B|R|B|R|B|R|B|
///   |-+-+-+-+-+-+-|
/// E |R|B|R|B|R|B|R|
///   |-+-+-+-+-+-+-|
/// F |R|B|R|B|R|B|R|
///   +-------------+
fn test_board() -> Board
{
    vec![vec![E, E, E, E, E, E, E],
         vec![E, E, E, E, E, E, E],
         vec![B, R, B, R, B, R, B],
         vec![B, R, B, R, B, R, B],
         vec![R, B, R, B, R, B, R],
         vec![R, B, R, B, R, B, R]]
}


/////////////////////////////////////////////
//
// Main game logic
//
/////////////////////////////////////////////

/// runs the application
///  * set up the board
///  * run the game
///  * print the result
fn main()
{
    let mut board = new_board();
    match run_game(&mut board)
    {
        R => {println!("\n\nRed won!");}
        B => {println!("\n\nBlack won!");}
        E => {println!("\n\ndraw!");}
    }
    print_board(&board);
}

/// Runs the Connect 4 game, and returns the winning player.
///
/// A game is played with two players.
///  * For each turn we ask the player to take a turn, 
///    and update the board
///  * Next we check for a winner.
///  * If that player won, then we return their piece.
///  * If they didn't win, then play passes to the next player.
///
/// # Arguments
/// 
/// * board - the state of the board
fn run_game(board : &mut Board) -> Piece
{
    let mut turn = 0;
    let mut winner : Option<Piece> = None;
    // Pert 2: write the code for running the game.

    // while there's no winner returned by check_winner
    while let None = winner 
    {
        if turn == 0
        {
            println!("Red's turn...");
            println!("Here's the board!");
            print_board(board);

            human_turn(board, R);
        }
        else if turn == 1
        {
            println!("Black's turn...");
            println!("Here's the board!");
            print_board(board);

            human_turn(board, B);
        }

        // increment turn by 1 mod 2: 0 == p1 turn, 1 == p2 turn
        turn = (turn + 1) % 2;

        // update our iteration condition variable with the winner check
        winner = check_winner(board);
    }

    // if we got to this point, winner == Some, so we should be safe to unwrap and return the player that won's piece
    return winner.unwrap();
}


/// Try to make a move,
/// If the use fails to enter a valid move,
/// then print the error and ask the user again.
///
/// When the user enters a valid move,
/// then update the board,
/// and return if there was a winner.
fn human_turn(board : &mut Board, piece : Piece) -> Option<Piece>
{
    let mut tries_left = 5;
    // as long as make_move is returning errors, keep prompting
    while let Err(x) = make_move(board, piece)
    {
        // handling for an indecisive / stupid player
        tries_left -= 1;
        if tries_left <= 0 
        {
            println!("You have run out of tries! Due to your indecision, your turn will be skipped."); 
            break;
        }
        
        // print error passed up by function stack
        println!("{}", x); 

        println!("You have {} tries left to make your move. If you run out, your turn will be skipped.", tries_left);
    }

    // return piece placed
    return Some(piece);
}

fn make_move(board : &mut Board, piece : Piece) -> Result<Piece, GameError>
{
    // prompt to take user input for move 
    // (and ensure column value is correct)
    let column = match try_move() 
    {
        Ok(m) => m,
        Err(e) => {return Err(e);}
    };

    // get lowest row in col
    // and update board
    match check_full(board, column) {
        Ok(row) => 
        {   
            board[row][column] = piece;
            return Ok(piece);
        }
        Err(e) => 
        {
            return Err(e);
        }
    }
}

// check to see if col is full.
// if not, return lowest row in selected column
fn check_full(board : &mut Board, column : Move) -> Result<usize, GameError>
{
    // gravity mechanic - finds first slot at bottom of column and updates board. 
    // If none exists, we return none.
    let mut i : usize = 6;
    while i > 0
    {
        if board[i-1][column] != E 
        {
            i -= 1;
            continue;
        }
        else 
        {
            return Ok(i-1);
        }
        
    }
    return Err(game_error("Column must not be full."));
}

/// check to see if there's a winner
/// If there is a winner, then return that player's piece.
/// If the game is a draw, then return E.
/// Otherwise return None.
///
/// # Arguments
///
/// * board - the state of the board
fn check_winner(board : &Board) -> Option<Piece>
{
    //Part 4: check to see if the board has a winner.
    //If there is a winner return Some(Piece).
    //
    //If there's a draw return Some(E).
    //
    //If there's no winner yet return None
    //
    
    let rows = 6;
    let cols = 7;
    let mut empty_found = false;
    // Vertical Check
    for col in 0..cols 
    {
        for row in 0..rows-3 
        {
            if board[row][col] == E
            {
                empty_found = true;
                continue;
            }

            if board[row][col] == board[row+1][col] 
            && board[row][col] == board[row+2][col] 
            && board[row][col] == board[row+3][col] 
            {
                return Some(board[row][col]);
            }
        }
    }

    // Horizontal check
    for row in 0..rows
    {
        for col in 0..cols-3
        {
            if board[row][col] == E
            {
                empty_found = true;
                continue;
            }

            if board[row][col] == board[row][col+1] 
            && board[row][col] == board[row][col+2] 
            && board[row][col] == board[row][col+3] 
            {
                return Some(board[row][col]);
            }
        }
    }

    // bottom-up diagonal
    for row in 0..rows-3
    {
        for col in 0..cols-3
        {
            if board[row][col] == E
            {
                empty_found = true;
                continue;
            }

            if board[row][col] == board[row+1][col+1] 
            && board[row][col] == board[row+2][col+2] 
            && board[row][col] == board[row+3][col+3] 
            {
                return Some(board[row][col]);
            }
        }
    }

    // top-down diagonal
    for row in 3..rows
    {
        for col in 0..cols-3
        {
            if board[row][col] == E
            {
                empty_found = true;
                continue;
            }

            if board[row][col] == board[row-1][col+1] 
            && board[row][col] == board[row-2][col+2] 
            && board[row][col] == board[row-3][col+3] 
            {
                return Some(board[row][col]);
            }
        }
    }

    if empty_found == false
    {
        // no empties found on board - all slots filled! Draw!
        return Some(E);
    }

    return None;
}

/////////////////////////////////////////////
//
// Code to ask for, and validate, a human player's move
//
/////////////////////////////////////////////

/// Ask the user to input a move,
/// and check that the move is valid.
/// Return the move if it's valid.
/// Return a GameError on faliure.
///
/// # Arguments
/// 
/// * board - the current State of the board
fn try_move() -> Result<Move, GameError>
{
    let line = ask_for_move()?;
    return validate_move(&line);
}

/// Ask the user for a move, retun the move is possible
fn ask_for_move() -> Result<String, GameError>
{
    print!("Select a square? ");
    stdout().flush().unwrap();
    let mut line = String::new();
    match stdin().read_line(&mut line)
    {
        Ok(_)  => {return Ok(line.trim().to_string());}
        Err(e) => {return Err(game_error(&e.to_string()));}
    }
}

/// Validate and parse the move
/// A move must be a single integer between 1 and 7
/// and C is one of 1, 2, 3, 4, 5, 6, 7
///
/// This will return OK(n) if n is a valid move.
/// Note this does not garuentee that the column has
/// space for a move, you need to do that yourself.
///
/// # Arguments
///
/// * board - the state of the board
///
fn validate_move(col : &str) -> Result<Move, GameError>
{
    let c = match col
            {
                "1" => Ok(0), 
                "2" => Ok(1), 
                "3" => Ok(2), 
                "4" => Ok(3), 
                "5" => Ok(4), 
                "6" => Ok(5), 
                "7" => Ok(6), 
                _   => Err(game_error("column must be one of 1-7"))
            }?;
    return Ok(c);
}


//////////////////////////////////////////////
//
// Game Error code: You don't need to worry about this.
//
// you can print a GameError (ge) with
// println!("{}", ge);
//
//////////////////////////////////////////////

/// A GameError is an error with Tic Tac Toe
#[derive(Debug, Clone)]
struct GameError { msg : String }

/// make a new game error
fn game_error(err : &str) -> GameError
{
    GameError{msg: err.to_string()}
}

impl Display for GameError
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        write!(f,"Game Error: {}",self.msg)
    }
}

impl Display for Piece
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        match self {
            Piece::R => write!(f, "R"),
            Piece::B => write!(f, "B"),
            Piece::E => write!(f, " "),
        }
    }
}

impl Error for GameError
{
    fn description(&self) -> &str
    {
        &self.msg
    }
}

