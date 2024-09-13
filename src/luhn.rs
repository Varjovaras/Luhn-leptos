use std::fmt::Display;

use rand::Rng;

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
pub const DEFAULT_LENGTH: u32 = 16;
// const ITERATIONS: u32 = 10_000_000;

#[derive(Debug, Clone)]
pub struct Luhn {
    string_length: u32,
    credit_card_number: String,
    end_result: u32,
    i: u32,
}

impl Luhn {
    #[must_use]
    fn new() -> Self {
        let mut luhn = Self {
            string_length: DEFAULT_LENGTH,
            credit_card_number: String::new(),
            end_result: 0,
            i: 0,
        };
        luhn.generate_valid_luhn_number();
        luhn
    }

    #[must_use]
    pub fn new_with_length(length: u32) -> Self {
        let mut luhn = Self {
            string_length: length,
            credit_card_number: String::new(),
            end_result: 0,
            i: 0,
        };
        luhn.generate_valid_luhn_number();
        luhn
    }

    #[must_use]
    pub fn get_length(self) -> u32 {
        self.string_length
    }

    pub fn change_length(&mut self, new_length: u32) {
        self.string_length = new_length;
        self.generate_valid_luhn_number();
    }

    pub fn generate_valid_luhn_number(&mut self) {
        let mut rng = rand::thread_rng();
        // let mut valid_count = 0;
        let mut i = 0;
        loop {
            let random_string: String = (0..self.string_length)
                .map(|_| DIGITS[rng.gen_range(0..DIGITS.len())])
                .collect();

            if is_credit_card(&random_string) {
                println!("i = {i}");
                self.credit_card_number = random_string;
                self.i = i;
                self.end_result = get_result(&self.credit_card_number);
                break;
            }

            i += 1;
        }
    }
}

impl Default for Luhn {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Luhn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
            (credit_card_number: ,

             how many iterations: {}, end_result: {})",
            // self.credit_card_number,
            self.i,
            self.end_result
        )
    }
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::unwrap_used)]
pub fn is_credit_card(s: &str) -> bool {
    let mut sum = 0;
    let mut double = false;

    for c in s.chars().rev() {
        if let Some(mut digit) = c.to_digit(10) {
            if double {
                digit *= 2;
                if digit > 9 {
                    digit -= 9;
                }
            }
            sum += digit;
            double = !double;
        } else {
            return false;
        }
    }

    sum % 10 == 0
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::unwrap_used)]
pub fn get_result(s: &str) -> u32 {
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

    modified_string
        .chars()
        .filter_map(|c| c.to_digit(10))
        .sum::<u32>()
}
