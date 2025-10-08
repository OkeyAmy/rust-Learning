## ⚙️ Rust Challenge: Temperature Converter with a Twist  

Welcome to my Rust learning challenge inspired by *The Rust Book – Chapter 3.1 (Variables and Mutability)*  
This challenge is focused on testing mastery of **constants**, **immutability**, **shadowing**, and **mutability** all within a single `fn main()` block  

---

### 🎯 Objective  
Build a compact, self-contained Rust program that demonstrates deep understanding of Rust’s variable binding system by handling multiple conversions and transformations **without using any external functions or modules**

---

### 🧩 Challenge Requirements  
All logic must be written **inside `fn main()`** — no helper functions or imports beyond `std::` if absolutely necessary  

#### **Step-by-step Tasks**
1. Declare a **constant** called `FACTOR` with a value of `5.0 / 9.0`  
2. Create an **immutable** variable `temp_f` with an initial value of `212.0`  
3. **Shadow** the `temp_f` variable twice:  
   - First, convert Fahrenheit to Celsius using the formula  
     ```rust
     temp_c = (temp_f - 32.0) * FACTOR
     ```  
   - Second, convert Celsius back to Fahrenheit using  
     ```rust
     temp_f = (temp_c * 9.0 / 5.0) + 32.0
     ```  
4. Create a **mutable** variable named `adjusted_temp` initialized with the latest `temp_f` value  
5. Perform a series of transformations on `adjusted_temp`:  
   - Add `10.0`  
   - Subtract `5.0`  
   - Multiply the result by `FACTOR`  
6. Use multiple `println!()` statements to clearly display the results of each step with output labels like:  
   ```text
   Original Fahrenheit: ...
   Converted to Celsius: ...
   Back to Fahrenheit: ...
   Adjusted Temperature: ...
