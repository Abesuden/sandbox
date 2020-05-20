// Arrays are a fixed list

use std::mem;

pub fn run() {

    let numOne: [i32; 5] = [1,2,3,4,5]; // [type; size of array]
    println!("{:?}", numOne); // use the debug to print the entire array
    println!("Index 1 is: {}", numOne[1]);
    
    let mut numTwo: [i32; 5] = [1,2,3,4,5]; // allows to reasign an array element
    numTwo[1] = 20;
    println!("Index 1 is: {}", numTwo[1]);

    // array functionality
    println!("Array length is: {}", numTwo.len()); // get length of array
    // println!("Allocation of array size of is: {}", std::mem::size_of_val(&numTwo)); // arrays are stack allocated
        //                                             `--> got ride of by adding the `use std::mem` at the top
    println!("Allocation of array size of is: {}", mem::size_of_val(&numTwo)); // arrays are stack allocated

    //            .----> I am not sure about the dereference here but I assume it is because it is being assigned a variable size
    let arrSlice: &[i32] = &numTwo; // dereference the array to point to the begining location to copy over
    let arrSlice2: &[i32] = &numTwo[0..2]; // [0..2] says, store indexes 0-2
    let arrSlice3: &[i32] = &numTwo[1..3];
    println!("Sliced: {:?}", arrSlice);
    println!("Sliced: {:?}", arrSlice2);
    println!("Sliced: {:?}", arrSlice3);
    let mut tester: [bool;3] = [true,true,true];
}