# Chapter 3: Common Programming Concepts

## Section 2: **Data Types**

* Rust is statically typed language, what does this mean? THis means that at compile time **ALL** variabl types must be known. 

## Scalar Types
* A scalar type represents a single value
    * Examples of these in rust are integers, floating-point numbers, booleans and characters

### Integer Types
* u32 => unsigned integer (only positive)
* i32 => signed integer
    * Each take up 32 bits of space

### Floating-Point Types
* f32 and f64
* Rust has two primitive types for floating-point numbers, which are numbers with decimal points. Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively. The default type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is capable of more precision. All floating-point types are signed.
```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

### Boolean Types

* Booleans are one byte in size. The Boolean type in Rust is specified using bool. For example:

```rust 
fn main() {
    let t = true;
    let f: bool = false; // with explicit type annotation
}
```

### Character Type
* Note that we specify char literals with single quotes, as opposed to string literals, which use double quotes. Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust. Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive. 

```rust 
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

### **Convert String to Integer Example**

```rust
fn main() {
    let guess = "42";
    let guess: u32 = guess.parse().expect("Not a number!")
    println!("The value of x is: {guess}");
}
```


## Scalar Types
* Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays
    
### Tuple
* A general way to group values together in one compound type
* Fixed length
```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // accessing the value we want
    let first_item = tup.0;
    let second_item = tup.1;
    let third_item = tup.2;
}
```
* Destructing a **tup**
```rust 
fn main() {
    let tup = (500, 6.4, 1);
// How to destructure
    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```

### Array 
* Unlike a tuple, every element of an array must have the same type. 
* Fixed length
* This is a way to maximize efficiency. Indicies greater than the length of the array will result in array. Other languages, these indicies are still accessible. 
```rust 
fn main() {
    let a = [1, 2, 3, 4, 5];
    //explicit
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3;5];
    // output is c = [3,3,3,3,
}
```





