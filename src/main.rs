fn main() {
   pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let board = build_board(n);
        let mut result: Vec<Vec<String>> = Vec::new();
        block_board(board, &mut result, 0);
        return result;
    }


    pub fn build_board(n: i32) -> Vec<Vec<String>> {
      let mut board = Vec::new();
        let mut i = 0;
        while i < n {
          let row = vec![String::from("."); n as usize];
          board.push(row);
          i += 1;
        } 
      board
    }

    
    pub fn block_board(mut board: Vec<Vec<String>>, result:  &Vec<Vec<String>>, pos: usize) {
        let mut count = 0;
        while count < board.len() {
            if count > 0 && board[pos][count - 1] == "Q" {
                board[pos][count - 1]  = ".".to_string();
            }
            if board[pos][count] != "X" {
                board[pos][count] = "Q".to_string();
                if pos == board.len() - 1 {
                    let joined: Vec<String> = board.iter().enumerate().map(|(_i, val)| {
                        let new_val: Vec<String> = val.iter().map(|x| if x == "X" {
                            ".".to_string()
                        } else {
                             x.clone()
                        }).collect();
                        new_val.join("")
                    }).collect();
                    println!("{:?}", joined);
                } else {
                    let new_board = update_block(&board.clone(), count as i32, pos as i32);
                    block_board(new_board, result, pos + 1);
                }
            }
            count += 1;
        }

    }
  
    pub fn update_block(board: &Vec<Vec<String>>, pos: i32, row: i32) -> Vec<Vec<String>> {
      let mut counter = 1;
      let new_board: Vec<Vec<String>> = board.iter().enumerate().map(|(i, val)| {
          if i as i32 > row {
                let mut new_row = board[i].clone();
                new_row[pos as usize] = "X".to_string();
            if (pos - counter) >= 0 {
                new_row[(pos - counter) as usize] = "X".to_string();
            }
            if (counter + pos) < board.len() as i32 {
                new_row[(pos + counter) as usize] = "X".to_string();
            }
            counter += 1;
            new_row
        }else {
            val.clone()
        }
      }).collect();
      new_board
    }

  solve_n_queens(4);

}
