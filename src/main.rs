use std::io;

fn main() {
    println!("\n*****************************");
    println!("    Temperature Converter");
    println!("*****************************");

    loop {
        println!("Do you want to convert to Celsius or to Farenheit? (C / F)");
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice = choice.trim();

        if choice == "C" {
            converter(false);
        } else if choice == "F" {
            converter(true);
        } else {
            continue;
        }
        break;
    }
}

fn converter(to_farenheit: bool) {
    let _init_type = if to_farenheit {"C"} else {"F"};
    let _new_type = if to_farenheit {"F"} else {"C"};

    loop {
        println!("Enter a temperature in {}:", _init_type);
        let mut _init_temp = String::new();

        io::stdin()
            .read_line(&mut _init_temp)
            .expect("Failed to read line");

        let _init_temp: f32 = match _init_temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let new_temp = if to_farenheit {
            _init_temp * 9.0 / 5.0 + 32.0
        } else {
            5.0 * (_init_temp - 32.0) / 9.0
        };

        println!("{}{} is the same as {:.2}{}", _init_temp, _init_type, new_temp, _new_type);
        break;
    }
}