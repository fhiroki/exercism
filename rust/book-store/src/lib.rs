pub fn lowest_price(books: &[u32]) -> u32 {
    let mut counts = vec![0; 5];
    let mut price = 0.0;

    for &x in books {
        counts[x as usize - 1] += 1;
    }

    while counts.iter().any(|&x| x != 0) {
        let mut cnt = 0;
        for i in 0..5 {
            if counts[i] > 0 {
                if books.len() % 8 == 0 && cnt == 4 {
                    continue;
                }
                cnt += 1;
                counts[i] -= 1;
            }
        }
        let discount = match cnt {
            1 => 1.0,
            2 => 0.95,
            3 => 0.9,
            4 => 0.8,
            5 => 0.75,
            _ => 1.0,
        };
        price += cnt as f32 * 8.0 * discount;
    }

    (price * 100.0).ceil() as u32
}
