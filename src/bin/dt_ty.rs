// fn main() {
//     let guess: "42".parse().expect("Not a number");
//     println!("The value of guess is: {guess}");
// }

// Here are the various data types in Rust:
// Length Interger : Signed : Unsigned
// 8 Bit :i8 : u8
// 16 Bit :i16 : u16
// 32 Bit :i32 : u32
// 64 Bit :i64 : u64
// 128 Bit :i128 : u128
// 256 Bit :i256 : u256

// What are the diff in signed and unsigned?
// Signed: the vale can be negative or positive
// Unsigned: the vale can only be positive

// // Floating
// fn main() {
//     let x = 2.0; // f64
//     let y: f32 = 3.0; // f32
//     println!("The value of x is: {x}");
//     println!("The value of y is: {y}");
// }

// Numeric Operations
fn main() {
    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let multiplication = 4 * 30;

    let division = 56.7 / 32.2;

    let remainder = 43 % 5;

    println!("Sum: {sum}");
    println!("Difference: {difference}");
    println!("Multiplication: {multiplication}");
    println!("Division: {division}");
    println!("Remainder: {remainder}");
}