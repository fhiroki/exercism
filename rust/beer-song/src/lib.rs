fn get_bottle(n: u32) -> String {
    if n == 0 {
        return String::from("No more bottles");
    } else if n == 1 {
        return String::from("1 bottle");
    }
    return format!("{} bottles", n);
}

pub fn verse(n: u32) -> String {
    let mut song = format!(
        "{} of beer on the wall, {} of beer.\n",
        get_bottle(n),
        get_bottle(n).to_lowercase()
    );
    if n == 0 {
        song += "Go to the store and buy some more, ";
    } else {
        song += &format!(
            "Take {} down and pass it around, ",
            if n == 1 { "it" } else { "one" }
        );
    }
    song += &format!(
        "{} of beer on the wall.\n",
        get_bottle(if n != 0 { n - 1 } else { 99 }).to_lowercase()
    );
    String::from(song)
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = String::new();
    for n in (end..start+1).rev() {
        song += &verse(n);
        if n != end {
            song += "\n";
        }
    }
    String::from(song)
}
