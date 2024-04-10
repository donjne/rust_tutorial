fn main() {
    let mut x = 5;
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("---------------------------------------");
    println!("Mutable, Immutable variables and constants:");
    println!("The value of three hours in seconds is: {THREE_HOURS_IN_SECONDS}");
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("---------------------------------------");
    println!("The shadowing exercise starts here:");
    shadowing();

    let spaces = "  ";
    let spaces = spaces.len();
    println!("The amount of spaces are: {}", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");

    }

    fn shadowing() {
        let x = 5;
        let x = x + 1;
        {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
        }
        println!("The value of x is: {x}");
        }
        
    
