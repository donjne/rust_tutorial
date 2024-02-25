// src/multiples.rs

pub fn sum_of_multiples(limit: u32) -> u32 {
    let mut total_sum = 0;

    for i in 1..limit {
        if i % 3 == 0 || i % 5 == 0 {
            total_sum += i;
        }
    }

    total_sum
}

pub fn main() {
    let limit = 1000;
    let result = sum_of_multiples(limit);
    println!("Sum of multiples below {}: {}", limit, result);
}
