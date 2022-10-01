use std::cmp::{min};

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let board = MineBoard::new(minefield);
    board.annotated()
}


struct MineBoard<'a> {
    board: &'a[&'a str],
    height: usize,
    width: usize,
}

impl<'a> MineBoard<'a> {
    fn new(board: &'a[&'a str]) -> Self {
        MineBoard {
            board,
            height: board.len(),
            width: board.first().unwrap_or(&"").len()
        }
    }

    fn annotated(&self) -> Vec<String> {
        let mut output: Vec<Vec<char>> = self.clone_board();
        for (row_num, row) in self.board.iter().enumerate() {
            for (col_num, c) in row.chars().enumerate() {
                if c.is_ascii_whitespace() {
                    let mine_count = self.block(row_num, col_num).join("").chars()
                        .filter(|&c| c == '*').count();
                    if mine_count > 0 {
                        output[row_num][col_num] = mine_count.to_string().chars().next().unwrap();
                    }
                }
            }
        }
        output.iter().map(String::from_iter).collect()
    }

    /// get a subsection of the board around a given row & column. Max size is 3x3
    fn block(&self, row: usize, col: usize) -> Vec<String> {
        let col_range = col.saturating_sub(1)..=min(self.width - 1, col + 1);
        (row.saturating_sub(1)..=min(row+1, self.height - 1))
            .map(|r| self.board[r][col_range.clone()].to_string())
            .collect()
    }

    fn clone_board(&self) -> Vec<Vec<char>> {
        self.board.iter().map(|&s| s.chars().collect()).collect()
    }
}
