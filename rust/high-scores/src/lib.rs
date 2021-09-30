#[derive(Debug)]
pub struct HighScores<'a>(&'a [u32]);

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores(scores)
    }

    pub fn scores(&self) -> &[u32] {
        self.0
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores().last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores().iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let n = if self.scores().len() < 3 {
            self.scores().len()
        } else {
            3
        };
        let mut v = self.scores().to_vec();
        v.sort_by(|a, b| b.cmp(a));
        v[0..n].to_vec()
    }
}
