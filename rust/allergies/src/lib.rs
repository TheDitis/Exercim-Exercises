use crate::Allergen::{Cats, Chocolate, Eggs, Peanuts, Pollen, Shellfish, Strawberries, Tomatoes};

pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

impl Allergen {
    pub const MAX: u32 = Self::Cats as u32;

    fn by_score(score: u32) -> Option<Allergen> {
        match score {
            1 => Some(Eggs),
            2 => Some(Peanuts),
            4 => Some(Shellfish),
            8 => Some(Strawberries),
            16 => Some(Tomatoes),
            32 => Some(Chocolate),
            64 => Some(Pollen),
            128 => Some(Cats),
            _ => None,
        }
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies().contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut num = u32::MAX;  // checking all square numbers, starting from max (almost)
        let mut output: Vec<Allergen> = Vec::new();
        let mut score = self.score;
        loop {
            num = (num as f64 / 2.0).ceil() as u32;  // divide num by 2, making sure it doesn't round down!
            if score == 0 { break output }  // if we've run out of remaining score, return output
            if num > score { continue }  // if num is out of scope of score, keep dividing in half
            // if the current num corresponds to an existing allergy, push it to output
            if let Some(allergen) = Allergen::by_score(num) {
                output.push(allergen);
            }
            score -= num; // subtract num from remaining score (it has been used up by an allergen)
        }
    }
}
