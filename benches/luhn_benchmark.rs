use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;

// Original implementation
fn original_is_credit_card(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let mut modified_string = String::new();
    for (i, &c) in chars.iter().enumerate() {
        if i % 2 == 0 {
            #[allow(clippy::unwrap_used)]
            let mut new_digit = c.to_digit(10).unwrap() * 2;
            if new_digit > 9 {
                new_digit = new_digit / 10 + new_digit % 10;
            }
            #[allow(clippy::unwrap_used)]
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

// Optimized implementation by claude 3.5 Sonnet
fn optimized_is_credit_card(s: &str) -> bool {
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

fn generate_random_cc_number(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| rng.gen_range(0..10).to_string())
        .collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let valid_cc = "4539319503436467";
    let invalid_cc = "1234567890123456";
    let long_cc = &generate_random_cc_number(10_000);

    c.bench_function("original_valid_16", |b| {
        b.iter(|| original_is_credit_card(black_box(valid_cc)));
    });
    c.bench_function("optimized_valid_16", |b| {
        b.iter(|| optimized_is_credit_card(black_box(valid_cc)));
    });

    c.bench_function("original_invalid_16", |b| {
        b.iter(|| original_is_credit_card(black_box(invalid_cc)));
    });
    c.bench_function("optimized_invalid_16", |b| {
        b.iter(|| optimized_is_credit_card(black_box(invalid_cc)));
    });

    c.bench_function("original_long_1000", |b| {
        b.iter(|| original_is_credit_card(black_box(long_cc)));
    });
    c.bench_function("optimized_long_1000", |b| {
        b.iter(|| optimized_is_credit_card(black_box(long_cc)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
