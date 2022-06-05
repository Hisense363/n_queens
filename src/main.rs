fn main() {
   pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let board = build_board(n);
        let mut result: Vec<Vec<String>> = Vec::new();
        block_board(board, &result, 0);
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

    
    pub fn block_board(mut board: Vec<Vec<String>>, result: &mut Vec<Vec<String>>, pos: usize) {
        let mut count = 0;
        while count < board.len() {
            if count > 0 && board[pos][count - 1] == "Q" {
                board[pos][count - 1]  = ".".to_string();
            }
            if board[pos][count] != "X" {
                board[pos][count] = "Q".to_string();
                if pos == board.len() - 1 {
                    let joined: Vec<String> = board.iter().enumerate().map(|(_i, val)| {
                        println!("5");
                        println!("{:?}", val);
                        val.join("")
                    }).collect();
                    result.push(joined);
                } else {
                    let new_board = board.clone();
                    block_board(new_board, result, pos + 1);
                }
            }
            count += 1;
        }

    }
  
    pub fn update_block(board: &Vec<Vec<String>>, pos: i32) -> Vec<Vec<String>> {
      let mut counter = 0 as usize;
      let pos = pos as usize;
      let new_board: Vec<Vec<String>> = board.iter().enumerate().map(|(_i, _val)| {
        let mut new_row = vec![".".to_string();board.len()];
        new_row[pos as usize] = "X".to_string();
        if counter > 0 && counter <= pos {
          new_row[(pos - counter)] = "X".to_string();
        }
        if counter > 0 && (counter + pos) < board.len() {
          new_row[pos + counter] = "X".to_string();
        }
        counter += 1;
        new_row
      }).collect();
      new_board
    }

  update_block(&vec![vec!["....".to_string()],vec!["....".to_string()],vec!["....".to_string()],vec!["....".to_string()]], 2);
  println!("{:?}", solve_n_queens(4));

}
