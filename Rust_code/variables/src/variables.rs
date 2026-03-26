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



}
