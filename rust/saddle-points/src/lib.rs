pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .fold(vec![], |mut points, (i, col)| {
            col.iter().enumerate().for_each(|(j, v)| {
                if col.iter().all(|x| v >= x) && (0..input.len()).all(|dy| v <= &input[dy][j]) {
                    points.push((i, j));
                }
            });
            points
        })
}
