fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn main() {
    let fahrenheit_temp: f64 = 98.6;
    let celsius_temp = fahrenheit_to_celsius(fahrenheit_temp);
    println!("{}°F is {}°C", fahrenheit_temp, celsius_temp);

    let celsius_temp: f64 = 37.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);
    println!("{}°C is {}°F", celsius_temp, fahrenheit_temp);
}
