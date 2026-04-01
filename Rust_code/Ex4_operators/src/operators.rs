/*
this is example 4 of rust which will cover operators in rust.
Operators in Rust are symbols that perform operations on variables and values. 
They can be categorized into several types, including:
1. Arithmetic Operators: +, -, *, /, %
2. Assignment Operators: =, +=, -=, *=, /=, %=
3. Comparison Operators: ==, !=, >, <, >=, <=
4. Logical Operators: &&, ||, !
5. Bitwise Operators: &, |, ^, <<, >>
6. Operator Precedence and Associativity

*/



fn main() {
    // 1. Arithmetic 
    /* 
    formula for arithmetic operators:
    key note: you can use variables or literal values with arithmetic operators

    addition: variable + variable
    subtraction: variable - variable
    multiplication: variable * variable
    division: variable / variable
    modulus: variable % variable

    */
    let a = 10;
    let b = 5;
    let sum = a + b; // addition
    let sum_literal = 10 + 5; // addition with literals
    let difference = a - b; // subtraction
    let product = a * b; // multiplication
    let quotient = a / b; // division
    let remainder = a % b; // modulus
    println!("Sum: {sum}, Sum (literal): {sum_literal}, Difference: {difference}, Product: {product}, Quotient: {quotient}, Remainder: {remainder}");

    // 2. Assignment Operators
    /* 
    formula for assignment operators:
    key note value acouls be a variable if it is already defined or a literal value
    assignment: variable = value
    addition assignment: variable += value
    subtraction assignment: variable -= value
    multiplication assignment: variable *= value
    division assignment: variable /= value
    modulus assignment: variable %= value


    */

    let mut c = 10;
    c += 5; // equivalent to c = c + 5
    println!("After += 5: {c}");
    c -= 3; // equivalent to c = c - 3
    println!("After -= 3: {c}");
    c *= 2; // equivalent to c = c * 2
    println!("After *= 2: {c}");
    c /= 4; // equivalent to c = c / 4
    println!("After /= 4: {c}");
    c %= 3; // equivalent to c = c % 3
    println!("After %= 3: {c}");

    //3. Comparison Operators
    /*
    formula for comparison operators:
    key note: comparison operators return a boolean value (true or false)

    equal to: variable == variable
    not equal to: variable != variable
    greater than: variable > variable
    less than: variable < variable
    greater than or equal to: variable >= variable
    less than or equal to: variable <= variable

    */
    let d = 10;
    let e = 5;
    let is_equal = d == e;
    let is_not_equal = d != e;
    let is_greater = d > e;
    let is_less = d < e;
    let is_greater_or_equal = d >= e;
    let is_less_or_equal = d <= e;
    println!("Is equal: {is_equal}");
    println!("Is not equal: {is_not_equal}");
    println!("Is greater: {is_greater}");
    println!("Is less: {is_less}");
    println!("Is greater or equal: {is_greater_or_equal}");
    println!("Is less or equal: {is_less_or_equal}");


    //4. Logical Operators
    /*
    formula:
    key note: logical operators are used to combine multiple boolean expressions
    logical operators in Rust work with boolean values

    logical AND: variable && variable
    logical OR: variable || variable
    logical NOT: !variable

    
    
    */

    let w = true;
    let x = false;

    let and_result = w && x; // logical AND
    let or_result = w || x; // logical OR
    let not_result = !w; // logical NOT
    println!("AND Result: {and_result}, OR Result: {or_result}, NOT Result: {not_result}");

    //5. Bitwise Operators
    /*
     these are used to perform bitwise operations on integers
     we use bitwise operators to manipulate individual bits of an integer value
     this is more of an advanced topic and is not used as often as the other operators 
     but is used a lot in low level programming and is important to understand if you want to do that type of programming
    
    formula:
    key note: bitwise operators work on the binary representation of integers
    bitwise AND: variable & variable
    bitwise OR: variable | variable
    bitwise XOR: variable ^ variable
    left shift: variable << number_of_bits
    right shift: variable >> number_of_bits
    */
    let bit1 = 0b1010; // binary representation of 10
    let bit2 = 0b1100; // binary representation of 12
    let bitwise_and = bit1 & bit2; // bitwise AND
    let bitwise_or = bit1 | bit2; // bitwise OR
    let bitwise_xor = bit1 ^ bit2; // bitwise XOR
    let left_shift = bit1 << 2; // left shift by 2 bits
    let right_shift = bit1 >> 2; // right shift by 2 bits
    println!("Bitwise AND: {bitwise_and}, Bitwise OR: {bitwise_or}, Bitwise XOR: {bitwise_xor}, Left Shift: {left_shift}, Right Shift: {right_shift}");

    //6. Operator Precedence and Associativity
    /* 
    precedence = which operation happens first
    associativity = which direction operations happen in when they are equal
    formula for operator precedence and associativity is:

    variable + variable * variable // multiplication happens first
    (variable + variable) * variable // parentheses happen first
    variable - variable - variable // left to right associativity
    variable / variable * variable // left to right when precedence is equal
    !variable && variable // ! happens before &&
    variable > variable && variable < variable // comparisons happen before &&
    variable == variable || variable == variable // comparisons happen before ||
    -variable * variable // negation happens first
    
    */
    let precedence_result_1 = 2 + 3 * 4;
    let precedence_result_2 = (2 + 3) * 4;
    let associativity_result_1 = 20 - 5 - 3;
    let associativity_result_2 = 20 / 5 * 2;
    let logical_precedence_1 = !false && true;
    let logical_precedence_2 = 10 > 5 && 3 < 7;
    let logical_precedence_3 = 10 == 5 || 8 == 8;
    let unary_precedence = -5 * 2;

    println!("2 + 3 * 4 = {precedence_result_1}");
    println!("(2 + 3) * 4 = {precedence_result_2}");
    println!("20 - 5 - 3 = {associativity_result_1}");
    println!("20 / 5 * 2 = {associativity_result_2}");
    println!("!false && true = {logical_precedence_1}");
    println!("10 > 5 && 3 < 7 = {logical_precedence_2}");
    println!("10 == 5 || 8 == 8 = {logical_precedence_3}");
    println!("-5 * 2 = {unary_precedence}");


}
