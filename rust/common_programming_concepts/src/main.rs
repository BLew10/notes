fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let guess  = "42";
    let guess: u32 = guess.parse().expect("Not a number!");
    println!("The value of x is: {guess}");

    another_function();
}

fn another_function() {
    println!("Another function.");
}