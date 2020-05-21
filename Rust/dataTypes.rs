/*
Primitive Types in Rust
-----------------------

Integers
    `--> u8, i8, u16, i16, u32, i32, u63, i64, u128, i128
    the numbers define the amount of bits to take in memory
    the letter means unsigned (u) or singed (i)
    
Floats
    `--> f32, f64

Booleans
    `--> bool

Characters
    `--> char

Tubles
    `--> 

Arrays
    `--> arefixed length, but vectors can "grow"...

*/

pub fn run () {

    // implicit types
    let x = 1; // by default it is "i32"
    let y = 3.14; // by default it is "f64"
    
    // explicit types
    let y: i64 = 100000;

    // find max size of the integer type
    println!("Max of i32 is: {}", std::i32::MAX);
    println!("Max of i32 is: {}", std::i64::MAX);
            //                      |   |    `--> looking for MAX
            //                      |   `--> data type
            //                      `--> standard library (think C++)


    // boolean
    let isTrue: bool = true;
    let isGreater: bool = 10 > 5;
    let mut isFlag = true;
    isFlag = false;
    
    // print with debug
    println!("{:?}", (x,y,isFlag));
    
    // char vars (they can be anything that is unicode)
    let var3 = 'a';
    // let face '\u{1F600}';// emojis are unicode but not on windows... only Mac
    // println!("{:?}", face);
}