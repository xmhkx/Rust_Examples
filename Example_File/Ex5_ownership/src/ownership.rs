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

    // you see the last example is not that good because we had to return the string back to the caller, 
    //and this is where references and borrowing come in, we will cover this in the next example :)

    let s1 = String::from("Hello"); // ommit this and see what happens, think why and how it relates to the last example in lines 40-43
    let length = better_calculate_length(&s1); // we are passing a reference to s1, s1 is still valid here
    println!("The length of '{s1}' is {length}"); // this works because s1 is still valid here

    let mut s1 = String::from("Hello"); // we need to make s1 mutable because we want to change it in the next example
    change_string(&mut s1); // we are passing a mutable reference to s1, s1 is still valid here
    println!("The modified string is: {s1}"); // this works because s1 is still valid here

    // this is not allowed in Rust, you cannot have two mutable references to the same data at the same time, this is to prevent data races
    // let mut string1 = &mut s1
    // let mut string2 = &mut s1

    let mut new_string = String::from("Hello"); 

    {
        let inner_string = &mut new_string; // this is allowed because inner_string is the only mutable reference to new_string in this scope
    }
    let another_string = &mut new_string; // this is allowed because inner_string is out of scope and no longer exists, cope


    let mut string1 = String::from("Hello");
    let string2 = &string1; // this is allowed because string2 is an immutable
    let string3 = &string1; // this is allowed because string3 is an immutable reference, we can have multiple immutable references to the same data
    // let string4 = &mut string1; // this is not allowed because we cannot have a mutable reference while we have immutable references

    let mut name = String::from("Alice");
    let name_ref1 = &name; // immutable reference to name
    let name_ref2 = &name; // another immutable reference to name
    println!("Name references: {name_ref1}, {name_ref2}"); // this works because we have multiple immutable references to the same data
    // name_ref1 and name_ref2 are not used after this point hence why we can have a mutable reference right after
    // remember borrow scopes are determined by the last usage of the reference, 
    // so after this line name_ref1 and name_ref2 are no longer used, we can have a mutable reference to name

    let name_change = &mut name; // mutable reference to name, this is allowed because we have no immutable references in this scope
    name_change.push_str(" Smith"); // change the name through the mutable reference
    println!("Modified name: {name}"); // this works because name is still valid here


}



fn take_a_string(s: String) { // string comes into scope right here <-
    println!("Taking ownership of the string: {s}");
} // string leaves scope here and is dropped and memory is freed

fn take_an_integer(i: i32) { // integer comes into scope right here <-
    println!("Taking ownership of the integer: {i}");
}// integer leaves scope here and is dropped

// usize is a unsigned integer type 
fn calculate_length(s: String) -> (String, usize){
    let length = s.len(); // calculate the length of the string

    (s, length) // return the string and its length as a tuple
}

// & is the reference see how we are using it
fn better_calculate_length(s: &String) -> usize { 
    s.len() // calculate the length of the string and return it
}

fn change_string(s: &mut String) { // &mut is a mutable reference, we can change the string through this reference
    s.push_str(", world!"); // change the string by appending ", world!" to it
}