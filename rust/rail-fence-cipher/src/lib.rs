pub struct RailFence(usize);

macro_rules! next {
    ( $self:ident, $i:ident, $j:ident, $k:ident ) => {
        if $i == 0 || $i == $self.0 - 1 {
            $j += 2 * ($self.0 - 1);
        } else if $k % 2 == 0 {
            $j += 2 * ($self.0 - 1 - $i);
        } else {
            $j += 2 * $i;
        }
        $k += 1;
    };
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence(rails as usize)
    }

    pub fn encode(&self, text: &str) -> String {
        let mut encoded = String::from("");
        for i in 0..self.0 {
            let (mut j, mut k) = (i, 0);
            while j < text.chars().count() {
                encoded.push(text.chars().nth(j).unwrap());
                next!(self, i, j, k);
            }
        }
        encoded
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut decoded = vec![' '; cipher.len()];
        let mut idx = 0;
        for i in 0..self.0 {
            let (mut j, mut k) = (i, 0);
            while j < cipher.chars().count() {
                decoded[j] = cipher.chars().nth(idx).unwrap();
                next!(self, i, j, k);
                idx += 1;
            }
        }
        decoded.iter().collect::<String>()
    }
}
