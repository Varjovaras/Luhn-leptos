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
