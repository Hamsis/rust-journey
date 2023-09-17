use std::io;

const DEGREE_FAHRENHEIT: &'static str = "°F";
const DEGREE_CELSIUS: &'static str = "°C";

fn main() {
    println!("FAHRENHEIT <=> CELSIUS");
    println!("Choisissez un mode de conversion");
    println!(" 1 - Fahrenheit vers celsius");
    println!(" 2 - Celsius vers fahrenheit");
    println!("----------------------------");

    let choix = loop {
        let mut choix = String::new();
        io::stdin()
            .read_line(&mut choix)
            .expect("Failed to read line");

        let choix = match choix.trim().parse::<u8>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choix == 1 || choix == 2 {
            break choix;
        } else {
            continue;
        }
    };

    println!("Quelle est la valeur de départ");
    let initial_value = loop {
        let mut choix = String::new();
        io::stdin()
            .read_line(&mut choix)
            .expect("Failed to read line");

        break match choix.trim().parse::<f64>() {
            Ok(num) => num,
            Err(_) => continue,
        };
    };

    println!("----------------------------");
    if choix == 1 {
        println!(
            "{initial_value} {DEGREE_FAHRENHEIT} = {} {DEGREE_CELSIUS}",
            fahrenheit_to_celsius(initial_value)
        );
    } else {
        println!(
            "{initial_value} {DEGREE_CELSIUS} = {} {DEGREE_FAHRENHEIT}",
            celsius_to_fahrenheit(initial_value)
        );
    };
}

fn fahrenheit_to_celsius(value: f64) -> f64 {
    (value - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(value: f64) -> f64 {
    (value * 9.0 / 5.0) + 32.0
}
