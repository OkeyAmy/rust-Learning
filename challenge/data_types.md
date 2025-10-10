## ⚙️ Rust Challenge 2: Mini Banking System  

This challenge is built to test complete understanding of **Rust data types** from *The Rust Book – Chapter 3.2 (Data Types)*  
It combines **scalars**, **tuples**, **arrays**, and **type annotations** into one system-level simulation  

---

### 🎯 Objective  
Build a mini banking system entirely inside `fn main()`  
The goal is to practice how Rust handles data storage typing safety and access through **tuples**, **arrays**, and **type inference** without using structs, loops, or external crates  

---

### 🧩 Challenge Requirements  

#### **1. Account Representation**
- Represent each user account as a **tuple** with three values:  
  ```rust
  let account1 = ("Alice", 1200.50, true);
  let account2 = ("Bob", 950.00, true);
  let account3 = ("Charlie", 0.00, false);
  ```

**Tuple structure**

* index `0`: `&str` → the user's name
* index `1`: `f32` → the account balance
* index `2`: `bool` → whether the account is active

---

#### **2. Store Accounts in an Array**

* Combine all three tuples into an array named `accounts`

  ```rust
  let accounts = [account1, account2, account3];
  ```
* This creates a **fixed-length array of tuples**

---

#### **3. Access and Print Account Details**

* Print the first and last account details individually using **tuple indexing** and **array indexing**

  ```rust
  println!("First account owner: {}", accounts[0].0);
  println!("First account balance: {}", accounts[0].1);
  println!("Last account owner: {}", accounts[2].0);
  println!("Last account active: {}", accounts[2].2);
  ```

---

#### **4. Deposit Simulation (Mutability & Type Safety)**

* Select the second account (`accounts[1]`)
* Extract the balance into a **mutable variable**:

  ```rust
  let mut balance = accounts[1].1;
  ```
* Increase the balance by `250.75`
* Print the new value using `println!`
* Do **not** modify the original array (Rust prevents direct mutation of tuple values inside an immutable array)
* This demonstrates how Rust enforces **immutability by default**

---

#### **5. Type Safety Check**

* Intentionally break type safety by assigning an incompatible type:

  ```rust
  // let balance: i32 = 100.0; // This should cause an error
  ```
* Observe the compiler message
* Then fix it by matching the correct type (`f32`)
* Note the difference and understand why Rust caught it before runtime

---

#### **6. Tuple Destructuring**

* Destructure one tuple into three separate variables:

  ```rust
  let (name, balance, active) = accounts[0];
  println!("{name} has a balance of {balance} active: {active}");
  ```
* This step helps you understand how to **extract** values from tuples directly

---

### 🧠 Bonus Round (Optional but Recommended)

* Create a **constant array** for interest rates:

  ```rust
  const RATES: [f32; 3] = [0.01, 0.015, 0.02];
  ```
* Manually calculate the new balances after applying each rate:

  ```rust
  let new_balance_1 = accounts[0].1 + accounts[0].1 * RATES[0];
  let new_balance_2 = accounts[1].1 + accounts[1].1 * RATES[1];
  let new_balance_3 = accounts[2].1 + accounts[2].1 * RATES[2];
  ```
* Print the results clearly to show how constants and arithmetic work together in typed expressions

---

### 🧰 **Rules**

* Everything must be written **inside `fn main()`**
* No external functions, structs, or libraries
* No loops, enums, or slices — only arrays and tuples
* Each variable must demonstrate **explicit or inferred typing**
* At least one intentional compile-time error must be tested and resolved

---

### 🏁 **Learning Goals**

By completing this challenge you will:
✅ Understand Rust's primitive and compound data types
✅ Practice using tuples and arrays together
✅ Master type inference vs explicit type annotation
✅ See how the compiler enforces strict type safety
✅ Learn destructuring and scope-based immutability
✅ Build the mental model of Rust's low-level data management

---

### 💻 **Code Placeholder**

You can start your solution here:

```rust
fn main() {
    // Write your code here
}
```

---

