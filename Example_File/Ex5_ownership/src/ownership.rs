/*
this is example 5 of rust which will cover ownership in rust :)

Go to the notes in this folder and read it to get a better understanding of ownership references/borrowing and slices in Rust. 

*/




fn main() {
    // 1. Ownership

    let s1 = String::from("Hello"); // s1 owns the string "Hello"
    let s2 = s1; // s2 takes ownership of the string, s1, no more s1
    println!("s2: {s2}"); // this works
    // println!("s1: {s1}"); // this will cause an error because

    let s1 = String::from("Swag"); // s1 owns the string "Swag"

    {
        let s2 = s1; // s2 takes ownership of the string, s1, no more s1
        println!("s2: {s2}"); // this works cause s2 is in inner scope
    }
    println!("s1: {s1}"); // this works because s1 is still in scope


    let x = 10;
    let y = x; // y copies the value of x, both x and y are valid
    println!("x: {x}, y: {y}"); // this works because x and y are both on the stack as a string is in the heap

    take_a_string(s1); // s1 is moved into the function, s1 is no more valid here
    //println!("s1: {s1}"); // this will cause an error because s1 is no more valid here

    take_an_integer(x); // x is copied into the function, x is still valid here
    // println!("x: {x}"); // this works because x is still valid here


    // 2. References and Borrowing

    

}


fn take_a_string(s: String) { // string comes into scope right here <-
    println!("Taking ownership of the string: {s}");
} // string leaves scope here and is dropped and memory is freed

fn take_an_integer(i: i32) { // integer comes into scope right here <-
    println!("Taking ownership of the integer: {i}");
}// integer leaves scope here and is dropped
