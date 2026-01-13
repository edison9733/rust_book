use std::io;

fn main() {
    println!("Temperature Converter");
    println!("=====================");
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");
    println!();
    println!("Enter your choice (1 or 2):");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid choice!");
            return;
        }
    };

    println!("Enter the temperature:");

    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read input");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid temperature!");
            return;
        }
    };

    match choice {
        1 => {
            let celsius = fahrenheit_to_celsius(temp);
            println!("{:.2}°F = {:.2}°C", temp, celsius);
        }
        2 => {
            let fahrenheit = celsius_to_fahrenheit(temp);
            println!("{:.2}°C = {:.2}°F", temp, fahrenheit);
        }
        _ => println!("Invalid choice! Please enter 1 or 2."),
    }
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}