/*

Welcome back this is example 2 which is funstions in Rust :)
functions are a fundamental so thats why will cover:
-syntax
-parameters
-return values
-experession vs statements

*/


fn main() {
    let a = 5;
    let b = 10;

    let sum = add(a, b);
    let product = multiply(a, b);

    println!("The sum of a and b is {}", sum);
    println!("The product of a and b is {}", product);
    hello();
}

/*

lets look at this function that adds two numbers

fn : this is the keyword that indicates we are defining a function
add : this is our name of the function
(x: i32, y: i32) : this is the parameter, and remember from example 1 that x and y are of the integer 32 type 
-> i32 : this is the return type( -> ) of the function, this tells us that it will only return an integer of 32 bit

*/

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

/*

expression vs statements:
your probably wondering why i'm leaving out the semicolon in the multiply function
This is on purpose, Because in Rust this is a expression and in this case its treated as a return value
so x + y; would be a error
a quick way to rember statements vs expressions is No semicolon = expression / Semicolon = statement !!

*/


fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

// functions like this can also have no parameters and return nothing, 
// this is called a unit type in Rust and is represented by ()
// this is just doing something without returning any value, in this case its just printing Hello World to the console
fn hello(){
    println!("Hello World");
}
