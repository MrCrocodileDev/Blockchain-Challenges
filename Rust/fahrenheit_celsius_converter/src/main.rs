use std::io;

fn main() {
    println!("Temperature converter (Fahrenheit/Celsius)");

    loop {
        println!("Choose your converter\n(1) - Fahrenheit to Celsius\n(2) - Celsius to Fahrenheit\n(3) - Quit");
        let mut converter = String::new();

        io::stdin()
            .read_line(&mut converter)
            .expect("Failed to read line");

        let converter: u32 = match converter.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if converter == 3 {
            println!("Goodbye!");
            break;
        }

        loop {
            println!("Your temperature :");
            let mut value = String::new();
            io::stdin()
                .read_line(&mut value)
                .expect("Failed to read line");
            let value: u32 = match value.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            match converter {
                1 => {
                    println!(
                        "{} Fahrenheit is equal to {} degrees Celsius",
                        value,
                        (value - 32) * 5 / 9,
                    );
                    break;
                }
                2 => {
                    println!(
                        "{} degrees Celsius is equal to {} Fahrenheit",
                        value,
                        (value * 5 / 9) + 32,
                    );
                    break;
                }
                _ => {
                    continue;
                }
            }
        }
    }
}
