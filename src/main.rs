use std::io::{self, Write};
use std::collections::HashMap;


// 1 == Celsius
// 2 == Fahrenheit
// 3 == Kelvin


fn main () {

    let mut collections = HashMap::new();
        collections.insert(("3", "1"), kelvin_to_celsius as fn (f64) -> f64);
        collections.insert(("1", "3"), celsius_to_kelvin as fn (f64) -> f64);
        collections.insert(("2", "1"), fahrenheit_to_celsius as fn (f64) -> f64);
        collections.insert(("1", "2"), celsius_to_fahrenheit as fn (f64) -> f64);
        collections.insert(("3", "2"), kelvin_to_fahrenheit as fn (f64) -> f64);
        collections.insert(("2", "3"), fahrenheit_to_kelvin as fn (f64) -> f64);

    let mut input_unit = String::new();
    let mut output_unit = String::new();
    let mut value = String::new();

    println!("Select the initial unit (enter the number)   1. Celsius   2. Fahrenheit   3. Kelvin");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_unit).unwrap();
    let input_unit = input_unit.trim();

    println!("Select the output unit (enter the number)   1. Celsius   2. Fahrenheit   3. Kelvin");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut output_unit).unwrap();
    let output_unit = output_unit.trim();

    println!("Enter the number of degrees you would like to convert");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut value).unwrap();
    let value = value.trim().parse().unwrap();

    let conversion_function = collections.get(&(input_unit, output_unit));

    match conversion_function {
        Some(func) => {
            let result = func(value);
            println!("Result: {} degrees", result);
        },
        None => {
          println!("Unsupported unit conversion! (try different units)");
        }
    };
}



fn kelvin_to_celsius (kelvin: f64) -> f64 {
  kelvin - 273.15
}

fn celsius_to_kelvin (celsius: f64) -> f64 {
  celsius + 273.15
}

fn fahrenheit_to_celsius (fahrenheit: f64) -> f64 {
  (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit (celsius: f64) -> f64 {
  (celsius * 9.0 / 5.0) + 32.0
}

fn kelvin_to_fahrenheit (kelvin: f64) -> f64 {
  (kelvin - 273.15) * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_kelvin (fahrenheit: f64) -> f64 {
  (fahrenheit + 32.0) * 5.0 / 9.0 + 273.15
}
