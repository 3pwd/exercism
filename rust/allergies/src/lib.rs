use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct Allergies(u32);

#[repr(u32)]
#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0 & (1 << *allergen as u32) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::iter()
            .filter(|allergen| self.is_allergic_to(allergen))
            .collect()
    }
}
