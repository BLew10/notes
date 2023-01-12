# Chapter 3: Common Programming Concepts

## Functions
* Defined by **fn**
```rust 
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

**How to Pass in Parameters**
```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```

### Statements vs Expressions
* Statements are instructions that perform some action and **do not** return a value.
* Expressions evaluate to a resultant value

    * Example of a statement
    ```rust 
    let y = 6
    ```
    * Since this doesnt return a value, you cannot assign another variable to it like so

    ```rust
    fn main() {
    let x = (let y = 6);
    }
    ```
    * Example of a expression 
    ```rust 
    5 + 6
    ```
    * As simple as it may be and not as obvious. It is an expression that returns 11
    
* Let's see another example
```rust
    fn main() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
    }
```

* This the code block between the brackets implicitly returns the value of 4. 
* Note that the **x + 1** line doesn’t have a semicolon at the end, which is unlike most of the lines you’ve seen so far. Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. Keep this in mind as you explore function return values and expressions next.

### Functions with Return Values

* Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow (->). In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly. Here’s an example of a function that returns a value:

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```

* Here is another example containing a parameter and return type 
```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```