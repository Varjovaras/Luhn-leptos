use std::fmt::Display;

use rand::Rng;

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const LENGTH: u32 = 16;
const ITERATIONS: u32 = 10_000_000;

#[derive(Debug, Clone)]
pub struct Luhn {
    string_length: u32,
    credit_card_number: String,
}

impl Luhn {
    #[must_use]
    pub fn new(s: &str) -> Self {
        Self {
            string_length: LENGTH,
            credit_card_number: s.into(),
        }
    }

    #[must_use]
    pub fn generate_valid_luhn_number(&mut self) -> Self {
        let mut rng = rand::thread_rng();
        // let mut valid_count = 0;
        let mut valid_number: Self = Self::new("");

        for i in 0..ITERATIONS {
            let random_string: String = (0..self.string_length)
                .map(|_| DIGITS[rng.gen_range(0..DIGITS.len())])
                .collect();

            if is_credit_card(&random_string) {
                println!("i = {i}");
                valid_number.credit_card_number = random_string;
                break;
            }
            // if i % 10_000 == 0 {
            //     println!("i = {i}");
            //     println!("valid numbers = {valid_count}");
            // }
        }
        valid_number
    }
}

impl Display for Luhn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.credit_card_number)
    }
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::unwrap_used)]
pub fn is_credit_card(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let mut modified_string = String::new();

    for (i, &c) in chars.iter().enumerate() {
        if i % 2 == 0 {
            let mut new_digit = c.to_digit(10).unwrap() * 2;
            if new_digit > 9 {
                new_digit = new_digit / 10 + new_digit % 10;
            }
            modified_string.push(char::from_digit(new_digit, 10).unwrap());
        } else {
            modified_string.push(c);
        }
    }

    let result = modified_string
        .chars()
        .filter_map(|c| c.to_digit(10))
        .sum::<u32>()
        .to_string();

    result.ends_with('0')
}
