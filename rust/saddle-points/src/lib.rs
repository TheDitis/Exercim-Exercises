pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let matrix = Matrix(input);
    matrix.saddle_points()
}


struct Matrix<'a>(&'a [Vec<u64>]);

impl<'a> Matrix<'a> {
    pub fn saddle_points(&self) -> Vec<(usize, usize)> {
        let mut output: Vec<(usize, usize)> = vec![];
        let row_maxs = self.row_maxs();
        let col_mins = self.col_mins();
        for (row_num, row) in self.0.iter().enumerate() {
            for (col_num, val) in row.iter().enumerate() {
                if val == &row_maxs[row_num] && val == &col_mins[col_num] {
                    output.push((row_num, col_num));
                }
            }
        }
        output
    }

    fn row_maxs(&self) -> Vec<u64> {
        self.0.iter().map(|row| *row.iter().max().unwrap_or(&0)).collect()
    }

    fn col_mins(&self) -> Vec<u64> {
        self.0.iter().enumerate().fold(vec![], |mut acc, (row_num, row)| {
            if row_num == 0 { acc.extend(row); }
            for (col_num, val) in row.iter().enumerate() {
                acc[col_num] = acc[col_num].min(*val);
            };
            acc
        })
    }
}