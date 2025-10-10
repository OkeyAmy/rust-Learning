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

// // Numeric Operations
// fn main() {
//     let sum = 5 + 10;

//     let difference = 95.5 - 4.3;

//     let multiplication = 4 * 30;

//     let division = 56.7 / 32.2;

//     let remainder = 43 % 5;

//     println!("Sum: {sum}");
//     println!("Difference: {difference}");
//     println!("Multiplication: {multiplication}");
//     println!("Division: {division}");
//     println!("Remainder: {remainder}");
// }

// Boolean
// fn main() {
//     let t = true;
//     let f: bool = false; // This is a way to explicitly type the variable as a boolean

//     println!("The value of t is: {t}");
//     println!("The value of f is: {f}");
// }


// // Compound Types
// // Tuple
// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     let k: (i32, f64, u8) = (2, 0.2, 3);

//     {
//         let b = [1, 2, 3];

//         let game_over = b[0];
//         let total = b[2] + b[1];

//         println!("The value of game_over is: {game_over}");
//         println!("The value of total is: {total}");

//     }

//     let win = k.0;
//     let loss = k.1;
//     let draw = k.2;

//     println!("The value of x is: {x}");
//     println!("The value of y is: {y}");
//     println!("The value of z is: {z}");

//     println!("The value of win is: {win}");
//     println!("The value of loss is: {loss}");
//     println!("The value of draw is: {draw}");
// }

// //  Testing

// use std::io;
// fn main() {
//    let a = [1, 2, 3, 4, 5];

//    println!("Please enter an array index.");

//    let mut index = String::new();

//    io::stdin().read_line(&mut index).expect("Failed to read line");

//    let index: usize = index.trim().parse().expect("Index entered was not a number");

//    let element = a[index];

//    println!("The value of the element at index {index} is: {element}");

// }


// Answer to the challenge 

fn main() {

   let account_1: (&str, f32, bool) = ("Alice", 1200.0, true);
   let account_2: (&str, f32, bool) = ("Bob", 1500.0, false);
   let account_3: (&str, f32, bool) = ("Charlie", 1800.0, true);


   let accounts = [account_1, account_2, account_3];

   

   println!("First account owner: {} and balance: {} and status state: {}", accounts[0].0, accounts[0].1, accounts[0].2 );
   println!("Second account owner: {} and balance: {} and status state: {}", accounts[1].0, accounts[1].1, accounts[1].2);
   println!("Third account owner: {} and balance: {} and status state: {}", accounts[2].0, accounts[2].1, accounts[2].2);


   let mut sec_balance: f32 = accounts[1].1;
   // let mut sec_status: i32 = accounts[1].1;

   sec_balance = sec_balance + 250.75;

   println!("Second account balance: {}", sec_balance);

   let (name, balance, status) = (accounts[0].0, accounts[0].1, accounts[0].2);
   println!("First account owner: {} and balance: {} and status state: {}", name, balance, status);

   const RATE: [f32;3] = [0.05, 0.1, 0.15];

   let new_bal_1 = accounts[0].1 + RATE[0];
   let new_bal_2 = accounts[1].1 + RATE[1];
   let new_bal_3 = accounts[2].1 + RATE[2];
   
   println!("First new account balance with rate increase: {}", new_bal_1);
   println!("Second new account balance with rate increase: {}", new_bal_2);
   println!("Third new account balance with rate increase: {}", new_bal_3);


}