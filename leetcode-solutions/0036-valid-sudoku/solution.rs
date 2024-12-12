use std::collections::HashSet;
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
            let mut rows: Vec<HashSet<char>> = vec![HashSet::new(); 9];
    let mut cols: Vec<HashSet<char>> = vec![HashSet::new(); 9];
    let mut boxes: Vec<HashSet<char>> = vec![HashSet::new(); 9];

    for i in 0..9 {
        for j in 0..9 {
            let cell = board[i][j];
            if cell == '.' {
                continue;
            }

            if rows[i].contains(&cell) {
                return false;
            }
            rows[i].insert(cell);

            if cols[j].contains(&cell) {
                return false;
            }
            cols[j].insert(cell);

            let box_index = (i / 3) * 3 + j / 3;
            if boxes[box_index].contains(&cell) {
                return false;
            }
            boxes[box_index].insert(cell);
        }
      }
      true
    }
}
