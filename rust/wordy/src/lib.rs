pub struct WordProblem;

pub fn answer(command: &str) -> Option<i32> {
    if !command.starts_with("What is") || !command.ends_with("?") {
        return None;
    }

    let command = command
        .replace("What is ", "")
        .replace("?", "")
        .replace("by ", "");
    let mut commands: Vec<&str> = command.split(" ").collect();
    commands.reverse();

    let mut ans = match commands.pop()?.parse::<i32>() {
        Ok(n) => n,
        Err(_) => return None,
    };

    while commands.len() != 0 {
        let op = commands.pop()?;
        if !vec!["plus", "minus", "multiplied", "divided"].contains(&op) {
            return None;
        }

        let num = commands.pop()?;
        match num.parse::<i32>() {
            Ok(x) => match op {
                "plus" => ans += x,
                "minus" => ans -= x,
                "multiplied" => ans *= x,
                "divided" => ans /= x,
                _ => return None,
            },
            Err(_) => return None,
        }
    }

    Some(ans)
}
