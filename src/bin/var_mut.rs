// // Variable Mutability
// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// Constants
// // Example 1
// fn main() {
//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//     println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
// }

// Shadowing
// Example 1
// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner block is: {x}");
//     }

//     println!("The value of x is: {x}");
// }

// Answer to the challenge
use std::io;


fn main() {

    println!("Enter a Fahrenheit temperature value:");
    let mut temp_f = String::new();
    io::stdin().read_line(&mut temp_f).expect("Failed to read line");

    const FACTOR: f32 = 5.0 / 9.0;
    let temp_f: f32 = temp_f.trim().parse().expect("Please type a number!");

        

    {
        let temp_c = (temp_f - 32.0) * FACTOR;
        let temp_f = (temp_c * 9.0 / 5.0) + 32.0;

        println!("Converted to Celsius: {temp_c}");
        
    }

    let mut adjusted_temp = temp_f;
    adjusted_temp = adjusted_temp + 10.0;
    adjusted_temp = adjusted_temp - 5.0;
    adjusted_temp = adjusted_temp * FACTOR;

    println!("Original Fahrenheit: {temp_f}");
    println!("Back to Fahrenheit: {temp_f}");
    println!("Adjusted Temperature: {adjusted_temp}");
}