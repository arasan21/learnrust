use std::io;

fn main() {
    println!("Welcome to Temperature Calculator");
    println!("1. Convert Fahrenheit to Celsius.\n2. Convert Celsius to Fahrenheit.");
    loop {
        println!("Please enter your option");
        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if option == 1 {
            loop {
                let mut fahrenheit = String::new();
                println!("Please enter value in  fahrenheit");
                io::stdin()
                    .read_line(&mut fahrenheit)
                    .expect("Failed to read line");

                let fahrenheit: f32 = match fahrenheit.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let celsius = fahrenheit_to_celsius(fahrenheit);
                println!(
                    "The value of {} fahrenheit  in celsius is {}",
                    fahrenheit, celsius
                );
                break;
            }
        } else if option == 2 {
            loop {
                let mut celsius = String::new();
                println!("Please enter value in celsius");

                io::stdin()
                    .read_line(&mut celsius)
                    .expect("Failed to read line");

                let celsius = match celsius.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let fahrenheit = celsius_to_fahrenheit(celsius);
                println!(
                    "The value of {} celsius in fahrenheit is {}",
                    celsius, fahrenheit
                );
                break;
            }
        } else {
            println!("Invalid option, Please enter valid option");
            continue;
        }
        break;
    }
}

fn fahrenheit_to_celsius(x: f32) -> f32 {
    // Temperature in degrees Celsius (°C) = (Temperature in degrees Fahrenheit (°F) - 32) * 5/9
    (x - 32.0) * (5.0 / 9.0)
}

fn celsius_to_fahrenheit(x: f32) -> f32 {
    //Temperature in degrees Fahrenheit (°F) = (Temperature in degrees Celsius (°C) * 9/5) + 32
    (x * (9.0 / 5.0)) + 32.0
}
