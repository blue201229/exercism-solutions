pub struct Allergies{
    score:u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
    Other,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self{score}
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let alg_score = Allergies::allerigen_to_score(allergen);
        self.score & alg_score > 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut alg_vec:Vec<Allergen> = Vec::new();
        let mut alg_score = 1;
        let mut iter_score = self.score;
        while iter_score > 0{
            if iter_score % 2 == 1 && alg_score<= 128{
                    alg_vec.push(Allergies::score_to_allerigen(alg_score));
            }
            iter_score /= 2;
            alg_score *= 2;
        }
        alg_vec
    }
    pub fn allerigen_to_score(allergen: &Allergen) -> u32{
        match allergen{
            Allergen::Eggs => 1,
            Allergen::Peanuts => 2,
            Allergen::Shellfish => 4,
            Allergen::Strawberries => 8,
            Allergen::Tomatoes => 16,
            Allergen::Chocolate => 32,
            Allergen::Pollen => 64,
            Allergen::Cats => 128,
            _ => 0,
        }
    }
    pub fn score_to_allerigen(score:u32) -> Allergen{
        match score{
            1 => Allergen::Eggs,
            2 => Allergen::Peanuts,
            4 => Allergen::Shellfish,
            8 => Allergen::Strawberries,
            16 => Allergen::Tomatoes,
            32 => Allergen::Chocolate,
            64 => Allergen::Pollen,
            128 => Allergen::Cats,
            _ => Allergen::Other,
        }
        
    }
}