use std::collections::HashMap;

pub fn tally(match_results: &str) -> String {
    let mut map: HashMap<String, Vec<u8>> = HashMap::new();
    for line in match_results.split('\n') {
        if line == "" {
            continue;
        }
        let words: Vec<&str> = line.split(';').collect();
        let team1 = words[0];
        let team2 = words[1];
        let result = words[2];

        // init means MP, W, D, L, P
        let init = vec![1, 0, 0, 0, 0];
        if let Some(x) = map.get_mut(team1) {
            x[0] += 1;
        } else {
            map.insert(team1.to_string(), init.clone());
        }
        if let Some(x) = map.get_mut(team2) {
            x[0] += 1;
        } else {
            map.insert(team2.to_string(), init.clone());
        }

        match result {
            "win" => {
                if let Some(x) = map.get_mut(team1) {
                    x[1] += 1;
                    x[4] += 3;
                }
                if let Some(x) = map.get_mut(team2) {
                    x[3] += 1;
                }
            }
            "loss" => {
                if let Some(x) = map.get_mut(team2) {
                    x[1] += 1;
                    x[4] += 3;
                }
                if let Some(x) = map.get_mut(team1) {
                    x[3] += 1;
                }
            }
            _ => {
                if let Some(x) = map.get_mut(team2) {
                    x[2] += 1;
                    x[4] += 1;
                }
                if let Some(x) = map.get_mut(team1) {
                    x[2] += 1;
                    x[4] += 1;
                }
            }
        }
    }

    let mut vec: Vec<_> = map.into_iter().collect();
    vec.sort_by(|x, y| {
        if y.1[4] == x.1[4] {
            return x.0.cmp(&y.0)
        }
        y.1[4].cmp(&x.1[4])
    });

    let mut matrix = String::from("Team                           | MP |  W |  D |  L |  P");
    for (k, v) in vec {
        matrix += &format!("\n{:30} |  {} |  {} |  {} |  {} |  {}", k, v[0], v[1], v[2], v[3], v[4]);
    }
    matrix.to_string()
}
