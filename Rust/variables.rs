pub fn run() {

    // create variable with let
    let name = "Rust";  // <--- these are like constants
    let number = 42; // <---------'
    
    let mut num = 42; // "mut" allows for it to be mutable
    num = 42-2;

    const ID: i32 = 001; // this is a constant and it needs to be defined for its type?
    //          `--> "integer 32 bit"
    
    let ( var1, var2 ) = ("Alpha", 3.14); // multiple variables at once
    
    // print variables
    println!("Program with {} and {} or {} or ID: {}", name, number, num, ID);
    println!("The {} is {}", var1, var2);
}