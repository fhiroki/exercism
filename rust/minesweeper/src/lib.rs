pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let h = minefield.len();
    if h == 0 {
        return vec![];
    }
    let w = minefield[0].len();

    let mut mines = vec![];
    for y in 0..h {
        let mut num = String::new();
        for x in 0..w {
            if minefield[y].chars().nth(x).unwrap() == '*' {
                num += "*";
            } else {
                let mut count = 0;
                for i in 0..3 {
                    let dy = (y as i32) + i - 1;
                    for j in 0..3 {
                        let dx = (x as i32) + j - 1;
                        if dy < 0 || dy >= h as i32 || dx < 0 || dx >= w as i32 {
                            continue;
                        }
                        if minefield[dy as usize].chars().nth(dx as usize).unwrap() == '*' {
                            count += 1;
                        }
                    }
                }

                if count == 0 {
                    num += " ";
                } else {
                    num += &count.to_string();
                }
            }
        }
        mines.push(num);
    }

    mines
}
