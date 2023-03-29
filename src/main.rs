use std::io;


fn main() {
    println!("Welcome to Jake's temperature calculator!");
    println!("Which unit would you like to input for conversion? (Enter number)");
    println!("1. Fahrenheit  2. Celsius  3. Kelvin");

    let mut input_unit = String::new();

    io::stdin()
        .read_line(&mut input_unit)
        .expect("Failed to read line");

    let input_unit = input_unit.trim();

    if input_unit == "1" {
        println!("You chose option 1");
    } else if input_unit == "2" {
        println!("You chose option 2");
    } else if input_unit == "3" {
        println!("You chose option 3");
    } else {
        println!("Please select one of the available options by number.");
    }


    println!("Which unit would you like to convert to? (Enter number)");
    println!("1. Fahrenheit  2. Celsius  3. Kelvin");

}
