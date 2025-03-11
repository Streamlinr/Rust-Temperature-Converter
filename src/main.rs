use std::io;
fn main() {
    //
    loop {
        // Select Message
        println!("Please Select Option, Control+C To Exit\nF to C [1]\nC to F [2]");

        // Select Mode //
        let mut select = String::new();
        // Read Input
        io::stdin().read_line(&mut select).expect("Failed");
        let select: f32 = match select.trim().parse() {
            // Make Sure Option Is On The List
            Ok(num) => num,
            Err(_) => continue,
        };

        // Catch Non Option
        if select > 2.0 {
            println!("Not An Option")
            // Fahrenheit To Celsius
        } else if select == 1.0 {
            fah_to_cel();
            break;
            // Celsius To Fahrenheit
        } else if select == 2.0 {
            cel_to_fah();
            break;
        };
        println!("{}", select)
    }
}

fn fah_to_cel() {
    // Get Degrees
    println!("Enter Degrees In Fahrenheit:");

    loop {
        // Select Mode //
        let mut degrees = String::new();
        // Read Input
        io::stdin().read_line(&mut degrees).expect("Failed");
        let degrees: f32 = match degrees.trim().parse() {
            // Make Sure Option Is On The List
            Ok(num) => num,
            Err(_) => continue,
        };

        // Convert Values
        let result = (degrees - 32.0) * 5.0 / 9.0;

        // Print Result
        println!(
            "{} Degrees Fahrenheit Is: {} Degrees Celsius",
            degrees, result
        )
    }
}

fn cel_to_fah() {
    // Get Degrees
    println!("Enter Degrees In Celsius:");

    loop {
        // Select Mode //
        let mut degrees = String::new();
        // Read Input
        io::stdin().read_line(&mut degrees).expect("Failed");
        let degrees: f32 = match degrees.trim().parse() {
            // Make Sure Option Is On The List
            Ok(num) => num,
            Err(_) => continue,
        };

        // Convert Values
        let result = (degrees * 1.8) + 32.0;

        // Print Result
        println!(
            "{} Degrees Celsius Is: {} Degrees Fahrenheit",
            degrees, result
        )
    }
}
