use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Ingredient {
    name: String,
    quantity: String,
}
impl Ingredient {
    pub fn new(name: &str, quantity: Quantity) -> Ingredient {
        Ingredient {
            name: name.to_owned(),
            quantity: quantity.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Quantity {
    amount: u32,
    unit: Unit,
}

impl Quantity {
    pub fn new(amount: u32, unit_type: UnitType) -> Quantity {
        return Quantity {
            amount: amount,
            unit: Unit::from(unit_type),
        };
    }

    pub fn to_string(&self) -> String {
        let amount_text: String = self.amount.to_string();
        let mut unit_text: String = self.unit.to_singular_string();
        if self.amount > 1 {
            unit_text = self.unit.to_plural_string();
        } else if self.unit.no_number_singular {
            return unit_text;
        }

        let sep = match self.unit.spaced {
            true => " ",
            false => "",
        };
        let quantity_text: String = vec![amount_text, unit_text].join(sep);
        quantity_text
    }
}

#[derive(Serialize, Deserialize)]
pub enum UnitType {
    GRAMS,
    MILLILITERS,
    OUNCES,
    PINCHES,
    HANDFULS,
}

#[derive(Serialize, Deserialize)]
struct Unit {
    unit_type: UnitType,
    no_number_singular: bool,
    spaced: bool,
}

impl Unit {
    pub fn from(unit_type: UnitType) -> Unit {
        match unit_type {
            UnitType::GRAMS => Unit {
                unit_type: UnitType::GRAMS,
                no_number_singular: false,
                spaced: false,
            },
            UnitType::MILLILITERS => Unit {
                unit_type: UnitType::MILLILITERS,
                no_number_singular: false,
                spaced: false,
            },
            UnitType::OUNCES => Unit {
                unit_type: UnitType::OUNCES,
                no_number_singular: false,
                spaced: false,
            },
            UnitType::PINCHES => Unit {
                unit_type: UnitType::PINCHES,
                no_number_singular: true,
                spaced: true,
            },
            UnitType::HANDFULS => Unit {
                unit_type: UnitType::HANDFULS,
                no_number_singular: true,
                spaced: true,
            },
        }
    }

    fn to_singular_string(&self) -> String {
        match self.unit_type {
            UnitType::GRAMS => "g".to_owned(),
            UnitType::MILLILITERS => "ml".to_owned(),
            UnitType::OUNCES => "oz".to_owned(),
            UnitType::PINCHES => "a pinch of".to_owned(),
            UnitType::HANDFULS => "a handful of".to_owned(),
        }
    }

    fn to_plural_string(&self) -> String {
        match self.unit_type {
            UnitType::GRAMS => self.to_singular_string(),
            UnitType::MILLILITERS => self.to_singular_string(),
            UnitType::OUNCES => self.to_singular_string(),
            UnitType::PINCHES => "pinches of".to_owned(),
            UnitType::HANDFULS => "handfuls of".to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantitative_quantity() {
        let quantity = Quantity::new(300, UnitType::GRAMS);
        assert_eq!(quantity.to_string(), "300g".to_owned());

        let quantity = Quantity::new(1, UnitType::MILLILITERS);
        assert_eq!(quantity.to_string(), "1ml".to_owned());
    }

    #[test]
    fn qualitative_quantity() {
        let quantity = Quantity::new(5, UnitType::PINCHES);
        assert_eq!(quantity.to_string(), "5 pinches of".to_owned());

        let quantity = Quantity::new(1, UnitType::HANDFULS);
        assert_eq!(quantity.to_string(), "a handful of".to_owned());
    }
}
