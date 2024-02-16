fn is_full(my_board: [[bool; 6]; 7]) -> bool {
   return my_board.iter().all(|&row| row.iter().all(|&cell| cell));
}

fn check_winner(my_board: [[bool; 6]; 7]) ->u8{ //1: yes, 0: no,

    for row in 0..6{ // check vertical connects
        if my_board[row][0] && my_board[row][1] && my_board[row][2] && my_board[row][3]{
            return 1;
        } else if my_board[row][2] && my_board[row][3] && my_board[row][4] && my_board[row][5] {
            return 1;
        } else if my_board[row][2] && my_board[row][3] && my_board[row][4] && my_board[row][5] {
            return 1;
        }
    }

    for col in 0..7{// check horizontal connects
        if my_board[0][col] && my_board[1][col] && my_board[2][col] && my_board[3][col]{
            return 1;
        } else if my_board[2][col] && my_board[3][col] && my_board[4][col] && my_board[5][col] {
            return 1;
        } else if my_board[3][col] && my_board[4][col] && my_board[5][col] && my_board[6][col] {
            return 1;
        }
    }

    //check diagonals
    for row in 0..3{ 
        for col in 0..4{
            if my_board[row][col] && my_board[row+1][col+1] && my_board[row+2][col+2] && my_board[row+3][col+3]{ //left bottom to right top
                return 1;
            }
            
        } 
    }
    return 0; //no matches, no winner here
}

fn evaluate_board(my_board: [[bool; 6]; 7]) -> u8 {
    //0: none, 1: player1, 2: player 2, 3: stalemate
    let mut result: u8 = check_winner(my_board);
    if result == 0 {//stalemate check
        if is_full(my_board){
            result = 3
        }
    }
    return result;
    
}
fn main() {
    let mut board: [[bool; 6]; 7] = [[true; 6]; 7];

    for n in 0..4{
        println!("{n}");
    }
    // Example: Set a specific cell to true
    board[3][4] = false;

    // Check if the board is full
    if is_full(board) {
        println!("The board is full.");
    } else {
        println!("The board is not full.");
    }
}
