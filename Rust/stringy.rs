// "Primitive str" <-- strings are immutable
// "String" <-- can grow, heap-allocated data structure
//      `--> use when you need to modify or own string data

pub fn run() {

    // create string
    let message = String::from("Hello Rust!");
    let mut mutMessage = String::from("Hello ");

    // String functionality
    println!("Length of string is: {}", message.len()); // get length of string
    mutMessage.push('R'); // add character to end of a mutable string
    mutMessage.push_str("ust!"); // add string to end of mutable string
    println!("{}", mutMessage);
    println!("Free space left: {}", mutMessage.capacity()); // prints the remaining space allocated
    println!("Is the string empty: {}", mutMessage.is_empty()); // prints if the string is empty
    println!("Is there a substing \"Rust\": {}", mutMessage.contains("Rust")); // prints the string has the substring
    let replacedMessage = message.replace("Rust", "World"); // replace a substring with another substring
    println!("{}", replacedMessage);

    // looping through 
    for word in mutMessage.split_whitespace() {
        println!("{}", word);
    }

    // set string to defined size
    let mut defString = String::with_capacity(10);
    defString.push('a');
    defString.push('b');

    // Assertion testing
    assert_eq!(2, defString.len()); // will only show anything if it fails
        // `--> does not fail because we pushed 'a' and 'b' earlier
    assert_eq!(3, defString.len()); // this will fail
    assert_eq!(10, defString.capacity()); // this will fail
    assert_eq!(11, defString.capacity()); // this will fail


    
}