pub struct PascalsTriangle(Vec<Vec<u32>>);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows = vec![];

        if row_count >= 1 {
            rows.push(vec![1]);
        }

        for i in 1..row_count as usize {
            let mut row: Vec<u32> = vec![];
            let prev_row = &rows[i - 1];
            row.push(1);
            for j in 0..prev_row.len() - 1 {
                row.push(prev_row[j] + prev_row[j + 1]);
            }
            row.push(1);
            rows.push(row);
        }

        PascalsTriangle(rows)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.0.to_vec()
    }
}
