/*

this is Example 1 going over variables in rust
what is covered in this example:

- how to declare variables
- mutability of variables
- types of variables
- shadowing of variables

*/

fn main() {
    // how to declare variables and mutability of variables
    // 
    let x = 5; 
    let mut y = 10; 
    y = 15;
    // x = 10; this would cause an error since x is not mutable

    // basic types of variables:
    
    // the : is used to specify the type of the variable

    let a: i32 = 20; // integer
    let b: f64 = 3.14; // floating point
    let c: bool = true; // boolean
    let d: char = 'R'; // character

    // Shadowing of variables:

    // IMPORTANT, pay antention if your a newbie to rust or programming in general
    // shadowing of variables

    let shadow = 50;

    {
        let shadow = shadow + 10; // this is shadowing the previous variable shadow
        println!("The value of shadow in the inner scope is: {}", shadow); // this will print 60
    }

    println!("The value of shadow in the outer scope is: {}", shadow); // this will print 50


    // tuples in rust, they are a collection of values of different types

    let tuple: (i32, f64, bool) = (10, 3.14, true); // this is a tuple with three values of different types
    let (x, y, z) = tuple; // this is destructuring the tuple into three variables x, y, z
    let first_element = tuple.0; // this is accessing the first element of the tuple, which is 10
    let second_element = tuple.1; // this is accessing the second element of the tuple, which is 3.14
    let third_element = tuple.2; // this is accessing the third element of the tuple, which is true
    println!("The value of x is: {}, y is: {}, z is: {}", x, y, z); // this will print the values of x, y, z

    // arrays in rust, they are a collection of values of the same type
    let array: [i32; 5] = [1, 2, 3, 4, 5]; // this is an array of 5 integers
    println!("The first element of the array is: {}", array[0]); // this will print the first element of the array, which is 1




}