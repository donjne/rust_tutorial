use std::io::{self, BufRead};


fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator: io::Lines<io::StdinLock<'_>> = stdin.lock().lines();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let _n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i64>().unwrap();
    }


}
