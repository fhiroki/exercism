pub struct Triangle(Vec<u64>);

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let mut v = sides.to_vec();
        v.sort();
        if v[0] == 0 || v[0] + v[1] < v[2] {
            return None;
        }
        Some(Triangle(v))
    }

    pub fn is_equilateral(&self) -> bool {
        self.0[0] == self.0[2]
    }

    pub fn is_scalene(&self) -> bool {
        self.0[0] != self.0[1] && self.0[1] != self.0[2]
    }

    pub fn is_isosceles(&self) -> bool {
        self.0[0] == self.0[1] || self.0[1] == self.0[2]
    }
}
