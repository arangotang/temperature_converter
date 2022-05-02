use std::io;

fn main() {
    println!("\n*****************************");
    println!("    Temperature Converter");
    println!("*****************************");
    for element in [false, true] {
        converter(element);
    }
}

fn converter(to_farenheit: bool) {
    let _init_type = if to_farenheit {"C"} else {"F"};
    let _new_type = if to_farenheit {"F"} else {"C"};

    loop {
        println!("\nEnter a temperature in {}:", _init_type);
        let mut _init_temp = String::new();

        io::stdin()
                .read_line(&mut _init_temp)
                .expect("Failed to read line");

        let _init_temp: i32 = match _init_temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let new_temp = if to_farenheit {
            _init_temp * 9 / 5 + 32
        } else {
            5 * (_init_temp - 32) / 9
        };

        println!("{}{} is the same as {}{}", _init_temp, _init_type, new_temp, _new_type);
        break;
    }
}