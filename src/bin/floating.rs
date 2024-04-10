fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("{x}");
    println!("{y}");

    numeric_operations();
    }

    fn numeric_operations() {
        // addition
        let sum = 5 + 10;
        // subtraction
        let difference = 95.5 - 4.3;
        // multiplication
        let product = 4 * 30;
        // division
        let quotient = 56.7 / 32.2;
        let truncated = -5 / 3; // Results in -1
        // remainder
        let remainder = 43 % 5;
        }

        fn booleans() {
            let t = true;
            let f: bool = false; // with explicit type annotation
            }
            
        fn characters() {
                let c = 'z';
                let z: char = 'ℤ'; // with explicit type annotation
                let heart_eyed_cat = '😻';
                }
                fn tuples() {
                    let tup: (i32, f64, u8) = (500, 6.4, 1);
                    let (x, y, z) = tup;
println!("The value of y is: {y}");

                    }

fn tuple_example_two() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    }

    fn array() {
        let anexample = [1, 2, 3, 4, 5];
        let a: [i32; 5] = [1, 2, 3, 4, 5];
        let a = [3; 5];
        }

        fn accessing_array() {
            let a = [1, 2, 3, 4, 5];
            let first = a[0];
            let second = a[1];
            }
        