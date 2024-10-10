fn is_solved(board: &[&[u8; 3]; 3]) -> i8 {
    let mut winner: i8 = 0;
    let mut is_finished: bool = true;
    
    // check for horizontal wins
    board.iter().for_each(|&row| {
        if &row[0] == &row[1] &&  &row[2] == &row[1] && &row[0] != &(0 as u8) {
            winner = row[0] as i8;
        };
        // check if there are empty fields
        if row.contains(&0) {
            is_finished = false;
        }
    });

    // check for vertical
    for (index, &field) in board[0].iter().enumerate() {
        if 
            &board[0][index] == &board[1][index] &&
            &board[2][index] == &board[1][index] &&
            &board[0][index] != &(0 as u8)
        {
            winner = field as i8;
        }
    };
    
    // check both diagonals
    if 
        (&board[0][0] == &board[1][1] && &board[2][2] == &board[1][1]) ||
        (&board[2][0] == &board[1][1] && &board[0][2] == &board[1][1]) &&
        &board[1][1] != &(0 as u8)
    {
        winner = board[1][1] as i8;
    }
    
    // return -1 if the board is not yet finished AND no one has won yet (there are empty spots)
    if winner == 0 && is_finished == false {
        return -1;
    }

    winner
}