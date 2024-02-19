fn is_full(my_board: [[[bool; 6]; 7]; 3]) -> bool {
   return my_board[2].iter().all(|&row| row.iter().all(|&cell| cell));
}

fn check_winner(my_board: [[bool; 6]; 7]) ->u8{ //1: yes, 0: no,

    for row in 0..7{ // check vertical connects
        if my_board[row][0] && my_board[row][1] && my_board[row][2] && my_board[row][3]{
            return 1;
        } else if my_board[row][1] && my_board[row][2] && my_board[row][3] && my_board[row][4] {
            return 1;
        } else if my_board[row][2] && my_board[row][3] && my_board[row][4] && my_board[row][5] {
            return 1;
        }
    }

    for col in 0..6{// check horizontal connects
        if my_board[0][col] && my_board[1][col] && my_board[2][col] && my_board[3][col]{
            return 1;
        } else if my_board[1][col] && my_board[2][col] && my_board[3][col] && my_board[4][col] {
            return 1
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
            if my_board[5-row][col] && my_board[4-row][col+1] && my_board[3-row][col+2] && my_board[2-row][col+3]{ //left top to right bottom
                return 1;
            } 
        } 
    }
    return 0; //no matches, no winner here
}

fn evaluate_board(my_board: [[[bool; 6]; 7]; 3]) -> u8 {
    //0: none, 1: player1, 2: player 2, 3: stalemate
    let mut result: u8 = check_winner(my_board[2]);
    if result == 0 {//stalemate check
        if is_full(my_board){
            result = 3
        }
    }
    return result;
    
}

fn possible_moves(my_board: [[[bool; 6]; 7]; 3]) -> u8 {
    let mut result: u8 = 0x7F;
    for col in 0..7{
        if my_board[2][col][5]{
            result &= !(1 << col);  
        }
    }
    return result;
}

fn make_move(mut my_board: &mut [[[bool; 6]; 7]; 3], col: usize, player_is_1: bool){
    let mut player_board: [[bool; 6]; 7];
    if player_is_1{
        player_board = my_board[0];
    } else {
        player_board = my_board[1];
    }

    //find highest point in the column, very crude and hardcodet divide and conquer algo
    let height: usize;
    if my_board[2][col][3] { //point > 3
        if my_board[2][col][4]{//point > 4 so its 5 as this is max height (remember height is 6, but 5 is max col)
            height = 5;
        } else {
            height = 4;
        }
    } else { //point < 3
        if my_board[2][col][1]{ //1 < point <= 3
            if my_board[2][col][2]{
                height = 3;
            } else {
                height = 2;
            }
        } else { // point <= 1
            if my_board[2][col][0]{
                height = 1;
            } else {
                height = 0;
            }
        }
    }

    //actually make a move, so change the arrays
    player_board[col][height] = true;
    my_board[2][col][height] = true;

}

fn get_set_bit_indices(value: u8) -> Vec<usize> { //thx Chat GPT
    let mut indices = Vec::new();

    for i in 0..8 {
        if (value & (1 << i)) != 0 {
            indices.push(i);
        }
    }

    indices
}

fn print_board(my_board: [[[bool; 6]; 7]; 3]){
    for btype in my_board{
        println!("\nBoard:\n");
        for x in btype{
            for y in x{
                print!("{y}|")
            }
            print!("\n------------------------------------\n")
        }
    }
}

fn main() {
    let mut board: [[[bool; 6]; 7]; 3] = [[[false; 6]; 7]; 3];

    while true{
        let board_state = evaluate_board(board);
        match board_state {
            0 => {}
            1 => {println!("Player 1 Wins"); break;}
            2 => {println!("Player 2 Wins"); break;}
            3 => {println!("Draw, Board is Full"); break;}
            _ => {println!("Error: False return Type");}
        }
        let moves = get_set_bit_indices(possible_moves(board));
        make_move(&mut board, 0, true);
        println!("Move: {}", moves[0]);
        print_board(board);
    }
}
