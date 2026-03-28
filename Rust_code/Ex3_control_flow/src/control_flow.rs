/*
This is example 3 and goes over control flow in Rust
What will be covered in this example:
- if statements
- else statements
- else if statements
- if used as an expression
- loop statements
- while statements
- for statements
- for with ranges

*/



fn main() {
    let testScore = 79;
// if statements
    if testScore >= 70 {
        println!("You passed!");
    } else {
        println!("You failed!");
    }

    let age = 21;
// if used as an expression
    let can_drink = if age >= 21 {"Can drink"} else {"Cannot drink"};
    println!("You: {can_drink}");

// loop
let mut count = 0;

loop{

    count += 1;
    println!("Counter: {count}");

    if count == 10{
        break;
    }
}
// loop with a return value

let mut counter = 0;

let result = loop{
    counter += 1;

    if counter == 10 {
        break counter * 2;
    }
};
println!("the value of result is {result}");


// while loop
let mut n = 3;
while n > 0{
    println!("{n}!");
    n -= 1;
}

// for loop with arrays
let array = [10, 20, 30, 40, 50];

for value in array{
    println!("the value in array is {value}");
}

// for with ranges

for i in 1..=6 {
    println!("the value of i is {i}");
}

}
