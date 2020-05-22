// tuples group together values of different types
// max of 12 elements

pub fn run() {
    let language: (&str, &str, i8) = ("Rust", "Programming", 24);

    println!("{} is a {} and it's ID is {}", language.0, language.1, language.2);
}