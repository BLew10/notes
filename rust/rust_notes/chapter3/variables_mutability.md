# Chapter 3: Common Programming Concepts

## Section 1: ****Variables and Mutability**

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```
* This will not compile, variable in rust are IMMUTABLE in nature, thus once bonded to a name, the value cannot change
* In order to make a variable mutable you must add the keyword 
* This prioritizes safety everything

```rust 
mut 
```

* Now this will compile and work

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```
### **Constants**

* You may be thinking, aren't constants immutable as well? Why do we need these things?
* Yes, they are the main difference is, you **CANNOT** use the key work **'mut'** and is declared using **'const'** not **'let'**
* Constants can be declared in any scope so they can be easily accessed
* Must be set to something static, all caps is convention
```rust
#![allow(unused)]
fn main() {
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}
```

### **Constants Summary**
* Constants are values that are bound to a name and cannot change.
* They are declared using the **'const'** keyword and **the type of the value must be annotated**. 
* Constants can be declared in any scope and can only be set to a constant expression, not a value that can only be computed at runtime. 
* Constants are always immutable and cannot be marked as mutable. 
* Constants are useful for values that many parts of the program need to know about and can be named to convey their meaning to future maintainers of the code. 
* They can also be used to store hardcoded values that might need to be updated in the future, since having them in one place makes it easier to change them.

### **Shadowing**
* Shadowing in a sense is re-declaring how variable will be defined
* The **'let'** keyword must be used
* When compiled, the compiler will see the second name of the variable, thus *overshadowing* the first 

```rust 
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```

* How is shadowing different than using **'mut'**?
    * Shadowing adds another layer of projection. Once a variable is declared **'mut'**, it is now mutable at any point. Shadowing makes it so it is so we are unable to reassign a variable unless **'let'** is used again, thus retain the security of immutability
    * Shadowing is essentially re-creating a new variable. Thus allowing us to change the type of the value but use the same name. 
    * Example: string -> i32
    ```rust
    fn main() {
    let spaces = "   ";
    let spaces = spaces.len();
    }
    ```


