// Assignment 1: Temperature Converter

const FREEZING_F_F64: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_F_F64) * (5.0 / 9.0)
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * (9.0 / 5.0)) + FREEZING_F_F64
}

fn main() {
    let mut temp_f: f64 = 32.0;

    // Convert the starting temperature
    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{:.0}°F = {:.2}°C", temp_f, temp_c);

    // Convert and print the next 5 integer temperatures
    let mut i: i32 = 1;
    while i <= 5 {
        temp_f = temp_f + 1.0; // next integer Fahrenheit
        let c = fahrenheit_to_celsius(temp_f);
        println!("{:.0}°F = {:.2}°C", temp_f, c);
        i = i + 1;
    }

    // (Extra example of the other direction, still within allowed concepts)
    let example_c: f64 = 0.0;
    let example_f = celsius_to_fahrenheit(example_c);
    println!("{:.2}°C = {:.2}°F", example_c, example_f);
}
