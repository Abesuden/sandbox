pub fn run() { // public function named run()
    
// printing
    // print to console
    println!("Hello Rust!");

    // print format
    println!("1 2 {0} 4 5 {1} {2} {2}", 3, 6, 8);

    // print argument with named parameters
    println!("This is how to print with {name}", name = "Rust");
    
    // placeholder traits (binary, hexadecimal, octal...)
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
    
    // print tupel for placeholder debug traits
    println!("{:?}", (12, true, "hello")); // prints all of these to the terminal
    //                `-> need these <-'
    
// Math
    println!("10 + 10 = {}", 10 + 10); // gets put into the "{}"

}