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
    //println!("s1: {s1}"); // this DOES not work, think why??


    let x = 10;
    let y = x; // y copies the value of x, both x and y are valid
    println!("x: {x}, y: {y}"); // this works because x and y are both on the stack as a string is in the heap

    let s1 = String::from("Hello"); // think why i added this again, delete it and see the error

    take_a_string(s1); // s1 is moved into the function, s1 is no more valid here
    //println!("s1: {s1}"); // this will cause an error because s1 is no more valid here

    take_an_integer(x); // x is copied into the function, x is still valid here
    // println!("x: {x}"); // this works because x is still valid here

    // this is a not so good way to do this, and references will show a better option 
    let s1 = String::from("Hello"); // s1 owns the string "Hello"
    let (s3, length) = calculate_length(s1); // s1 is moved into the function, s1 is no more valid here, s2 takes ownership of the string, length is the length of the string
    println!("The length of '{s3}' is {length}"); // this works because s2 is still valid here

    // 2. References and Borrowing

    




}



fn take_a_string(s: String) { // string comes into scope right here <-
    println!("Taking ownership of the string: {s}");
} // string leaves scope here and is dropped and memory is freed

fn take_an_integer(i: i32) { // integer comes into scope right here <-
    println!("Taking ownership of the integer: {i}");
}// integer leaves scope here and is dropped

fn calculate_length(s: String) -> (String, usize){
    let length = s.len(); // calculate the length of the string

    (s, length) // return the string and its length as a tuple
}