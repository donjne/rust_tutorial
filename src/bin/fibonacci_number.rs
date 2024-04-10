fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        let mut prev = 0;
        let mut curr = 1;
        for _ in 2..=n {
            let next = prev + curr;
            prev = curr;
            curr = next;
        }
        return curr;
    }
}

fn main() {
    for i in 1..=50 {
        let result = fibonacci(i);
        let ordinal = match i {
            1 => "1st".to_string(),
            2 => "2nd".to_string(),
            3 => "3rd".to_string(),
            11 => "11th".to_string(),
            12 => "12th".to_string(),
            13 => "13th".to_string(),
            _ => {
                let last_digit = i % 10;
                match last_digit {
                    1 => format!("{}st", i),
                    2 => format!("{}nd", i),
                    3 => format!("{}rd", i),
                    _ => format!("{}th", i),
                }
            }
        };
        println!("The {} Fibonacci number is: {}", ordinal, result);
    }
}
