use std::io;

fn is_full(my_board: [[bool; 6]; 7]) -> bool {
   return my_board.iter().all(|&row| row.iter().all(|&cell| cell));
}

fn check_vertical_connections(my_board: [[bool; 6]; 7]) ->bool{
    for row in 0..7{
        if my_board[row][0] && my_board[row][1] && my_board[row][2] && my_board[row][3]{
            return true;
        } else if my_board[row][1] && my_board[row][2] && my_board[row][3] && my_board[row][4] {
            return true;
        } else if my_board[row][2] && my_board[row][3] && my_board[row][4] && my_board[row][5] {
            return true;
        }
    }  
    return false;
}

fn check_horizontal_connections(my_board: [[bool; 6]; 7]) ->bool{
    for col in 0..6{// check horizontal connects
        if my_board[0][col] && my_board[1][col] && my_board[2][col] && my_board[3][col]{
            return true;
        } else if my_board[1][col] && my_board[2][col] && my_board[3][col] && my_board[4][col] {
            return true
        } else if my_board[2][col] && my_board[3][col] && my_board[4][col] && my_board[5][col] {
            return true;
        } else if my_board[3][col] && my_board[4][col] && my_board[5][col] && my_board[6][col] {
            return true;
        }
    }
    return false;
}

fn check_diagonal_connections(my_board: [[bool; 6]; 7]) ->bool {
    for row in 0..3{ 
        for col in 0..4{
            if my_board[row][col] && my_board[row+1][col+1] && my_board[row+2][col+2] && my_board[row+3][col+3]{ //left bottom to right top
                return true;
            }
            if my_board[5-row][col] && my_board[4-row][col+1] && my_board[3-row][col+2] && my_board[2-row][col+3]{ //left top to right bottom
                return true;
            } 
        } 
    }
    return false;
}

fn check_winner(my_board: [[bool; 6]; 7]) ->bool{ 
    return check_vertical_connections(my_board) || check_horizontal_connections(my_board) || check_diagonal_connections(my_board)
}

fn evaluate_board(my_board: [[[bool; 6]; 7]; 3]) -> u8 {
    //0: none, 1: player1, 2: player 2, 3: stalemate
    if check_winner(my_board[0]){
        return 1;
    }
    if check_winner(my_board[1]) {
        return 2;
    }
    if is_full(my_board[2]){//stalemate check
        return 3;
    }
    return 0;
}

fn possible_moves(my_board: [[[bool; 6]; 7]; 3]) -> Vec<usize> {
    let mut indices = Vec::new();
    for col in 0..7{
        if !my_board[2][col][5]{
            indices.push(col);
        }
    }
    return indices;
}

fn make_move_new_board(mut my_board: [[[bool; 6]; 7]; 3], col: usize, player_is_1: bool) -> [[[bool; 6]; 7]; 3]{//TODO: tests
    make_move(&mut my_board, col, player_is_1);
    return my_board;
}

fn find_highest_point(my_board: &mut [[[bool; 6]; 7]; 3], col: usize) -> usize{//TODO tests
    //find highest point in the column, very crude and hardcodet divide and conquer algo
    if my_board[2][col][3] { //point > 3
        if my_board[2][col][4]{//point > 4 so its 5 as this is max height (remember height is 6, but 5 is max col)
            return 5;
        } else {
            return 4;
        }
    } else { //point < 3
        if my_board[2][col][1]{ //1 < point <= 3
            if my_board[2][col][2]{
                return 3;
            } else {
                return 2;
            }
        } else { // point <= 1
            if my_board[2][col][0]{
                return 1;
            } else {
                return 0;
            }
        }
    }
}

fn make_move(my_board: &mut [[[bool; 6]; 7]; 3], col: usize, player_is_1: bool){//TODO tests
    let height: usize;
    height = find_highest_point(my_board, col);

    if player_is_1{
        my_board[0][col][height] = true;
    } else {
        my_board[1][col][height] = true;
    }
    my_board[2][col][height] = true;

}

fn print_board(my_board: [[[bool; 6]; 7]; 3]){
    println!("\nBoard:\n");
    for y in 0..6{
        for x in 0..7{
            if my_board[0][x][5-y] && my_board[2][x][5-y]{
                print!("  X  |")
            } else  if my_board[1][x][5-y] && my_board[2][x][5-y]{
                print!("  O  |")
            } else if !my_board[2][x][5-y]{
                print!("     |")
            } else{
                print!("corrupted")
            }
                

        }
        print!("\n------------------------------------------\n")
    }

}

fn player_move(possible_moves: Vec<usize>) -> usize {
    loop {
        let mut player_move_str = String::new();

        println!("Please choose a column (0-6):");
        io::stdin().read_line(&mut player_move_str)
            .expect("Failed to read line.");

        // Trim whitespace and remove newline character
        let player_move_str = player_move_str.trim();

        // Attempt to parse input to usize
        match player_move_str.parse::<usize>() {
            Ok(result) => {
                if possible_moves.contains(&result) {
                    return result;
                } else {
                    println!("Invalid move. Please choose from available columns.");
                }
            }
            Err(_) => {
                println!("Error: Invalid input. Please enter a number.");
            }
        }
    }
}
/*
fn minimax(my_board: [[[bool; 6]; 7]; 3], player_is_1: bool) -> u8{
    let board_eval = evaluate_board(my_board);
    if board_eval == 1 || board_eval == 2 || board_eval == 3{
        return board_eval;
    }
    if player_is_1{
        best_eval = 2;
        for i in possible_moves(my_board){
            if evaluate_board(make_move_new_board(my_board, i, player_is_1)) 
        }
    }
    else {
        best_eval = 1;
    }
}
 */

fn main() {
    let mut board: [[[bool; 6]; 7]; 3] = [[[false; 6]; 7]; 3];

    loop{
        let board_state = evaluate_board(board);
        match board_state {
            0 => {}
            1 => {println!("Player 1 Wins"); break;}
            2 => {println!("Player 2 Wins"); break;}
            3 => {println!("Draw, Board is Full"); break;}
            _ => {println!("Error: False return Type");}
        }

        let moves = possible_moves(board);
        make_move(&mut board, player_move(moves), true);
        print_board(board);

        let moves = possible_moves(board);
        make_move(&mut board, moves[0], false);
        println!("Move: {}", moves[0]);
        print_board(board);
    }
}
