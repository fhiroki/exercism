enum Direct {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size == 0 {
        return vec![];
    }

    let n = size as usize;
    let mut matrix = vec![vec![0; n]; n];
    let mut direct = Direct::RIGHT;

    let (mut i, mut j, mut v) = (0, 0, 1);

    loop {
        matrix[i][j] = v;
        if v == size * size {
            break;
        }
        v += 1;

        match direct {
            Direct::RIGHT => {
                if j == n - 1 || matrix[i][j + 1] != 0 {
                    direct = Direct::DOWN;
                    i += 1;
                } else {
                    j += 1;
                }
            }
            Direct::DOWN => {
                if i == n - 1 || matrix[i + 1][j] != 0 {
                    direct = Direct::LEFT;
                    j -= 1;
                } else {
                    i += 1;
                }
            }
            Direct::LEFT => {
                if j == 0 || matrix[i][j - 1] != 0 {
                    direct = Direct::UP;
                    i -= 1;
                } else {
                    j -= 1;
                }
            }
            Direct::UP => {
                if i == 0 || matrix[i - 1][j] != 0 {
                    direct = Direct::RIGHT;
                    j += 1;
                } else {
                    i -= 1;
                }
            }
        }
    }

    matrix
}
