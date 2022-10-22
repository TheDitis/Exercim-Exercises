
pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let rows = (0..row_count).fold(vec![], |mut rows, i| {
            rows.push(
                Self::new_row(rows.get((i as isize - 1) as usize)
                    .unwrap_or(&vec!())
            ));
            rows
        });
        PascalsTriangle { rows }
    }

    pub fn _new(row_count: u32) -> Self {
        let mut rows = vec![];
        (0..row_count).for_each(|i| {
            rows.push(
                Self::new_row(rows.get((i as isize - 1) as usize)
                    .unwrap_or(&vec!())
            ));
        });
        PascalsTriangle { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }

    fn new_row(prev_row: &Vec<u32>) -> Vec<u32> {
        if prev_row.is_empty() { return vec![1] }
        let mut output = vec![];
        for i in 0..prev_row.len()+1 {
            output.push(
                prev_row.get((i as isize - 1) as usize).unwrap_or(&0) +
                    prev_row.get(i).unwrap_or(&0)
            )
        }
        output
    }
}
