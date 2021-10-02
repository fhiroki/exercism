use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub struct Allergies {
    allergen: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Clone, FromPrimitive)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(mut score: u32) -> Self {
        let mut allergen = vec![];
        for i in (0..10).rev() {
            let v = 2u32.pow(i);
            if score >= v {
                match FromPrimitive::from_u32(v) {
                    Some(a) => allergen.push(a),
                    None => (),
                }
                score -= v;
            }
        }
        Allergies { allergen }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies().contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        (*self.allergen).to_vec()
    }
}
