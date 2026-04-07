# Rust Notes: Ownership, References, Borrowing, and Slices

## Ownership

Ownership is Rust's system for managing memory safely without needing a garbage collector. Instead of letting many parts of a program freely control the same data, Rust makes it very clear who is responsible for each value.

Every value in Rust has an owner. The owner is the variable or place in the program that is currently responsible for that value. When the owner goes out of scope, Rust automatically cleans up the value. This is why Rust can avoid both memory leaks and accidental use of freed memory in many common situations.

The main idea is simple: one value has one owner at a time. If ownership is passed somewhere else, the original owner can no longer use that value. Rust calls this a move.

This matters most for data stored on the heap, such as `String`, because heap data needs clear cleanup rules. Simple fixed-size types like integers are usually copied instead of moved, so they feel much easier to pass around.

### Important idea

Ownership is not just a syntax rule. It is Rust's way of answering this question:

"Who is responsible for this data right now?"

If Rust always knows that answer, it can keep memory safe.

## Move, Copy, and Clone

When a value is moved, responsibility for that value is transferred to a new owner. After that, the old variable is no longer valid.

Some types are so small and simple that Rust copies them instead. Types like `i32`, `bool`, and `char` are usually copied because duplicating them is cheap and safe.

If you want a full duplicate of heap data, you use `clone()`. This creates a separate copy of the data, rather than transferring ownership.

### Quick mental model

- `move` = give the value to someone else
- `copy` = duplicate a simple value
- `clone` = make a full duplicate of more complex data

## References and Borrowing

Sometimes you want to use a value without taking ownership of it. That is where references come in.

A reference lets you look at a value that belongs to something else. This is called borrowing. Borrowing is useful because it allows functions to work with data without permanently taking it away from the caller.

For example, if a function only needs to read a string, it should usually borrow the string instead of taking ownership of it. That way, the original variable can still be used afterward.

### Immutable borrowing

An immutable reference lets you read data, but not change it. You can have many immutable references at the same time because reading shared data is safe.

### Mutable borrowing

A mutable reference lets you change the original value. Rust is strict here: if data is being changed, Rust wants to make sure nothing else is using it at the same time in a conflicting way.

That is why Rust allows only one mutable reference at a time. This rule helps prevent bugs like accidental overwrites, conflicting changes, and data races.

## Borrowing Rules

Rust's borrowing rules are there to protect memory safety:

1. You can have many immutable references.
2. You can have one mutable reference.
3. You cannot mix mutable and immutable references to the same value at the same time.

These rules may feel strict at first, but they remove a whole category of bugs that are common in other languages.

## Why Ownership and Borrowing Matter

Ownership and borrowing work together:

- ownership decides who is responsible for the data
- borrowing allows temporary access without transferring responsibility
- mutable borrowing allows safe changes without losing control of the original value

This is one of the main reasons Rust programs can be both fast and safe. Rust gives low-level control over memory, but still checks a lot of dangerous situations at compile time.

## Slices

A slice is a reference to part of a collection instead of the whole thing. Slices do not own the data. They only point to an existing section of it.

This is useful when you want to work with part of a string or part of an array without creating a new copy.

For strings, the slice type is `&str`. A string slice points to some portion of string data. For arrays, a slice points to a range of elements.

### Why slices are useful

Slices make code more flexible and efficient because:

- they avoid unnecessary copying
- they let functions accept part of some data
- they keep a connection to the original value

A slice is basically Rust's way of saying:

"Use this part of the data, but do not take ownership of it."

## Small Examples

```rust
let name = String::from("Rust");
let first = &name[0..2];
```

Here, `first` is a slice. It does not own the string. It only refers to part of it.

```rust
let s = String::from("hello");
let len = s.len();
```

Calling `len()` only borrows the string. It does not take ownership.

## Summary

Ownership is Rust's way of tracking responsibility for data. References and borrowing let code use data without taking it over. Mutable borrowing allows safe changes, but under strict rules. Slices let you refer to part of data without copying it.

If you remember one big idea, remember this:

Rust is always trying to make memory access clear, safe, and predictable.
