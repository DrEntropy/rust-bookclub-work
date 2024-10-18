# Rust Book Club

This readme will capture lessons learned / etc from the Book and the Book club. 
Code will be in various folders at this level. 

## Chapter 1
This was just a chapter about setting up and making sure rust runs.
  `hello_cargo` is the first (cargo based) program that we make. 

## Chapter 2
This chapter has the reader program a guessing game in rust, 
just to see a whole working program and to get a feel for the language.
It covers `match`, `loop`, `break`, `continue`, `Result` and `expect` as 
well as using crates like `rand` to generate random numbers.  
All of these things will be covered in more detail in later chapters.

## Chapter 3
* Covers programming concepts that rust shares with other languages. "Common Programming Concepts"
* Variables and Mutability
    * Variables are immutable by default. Use `mut` to make them mutable.
    * Constants: Constants are always immutable. They are declared with `const` and must have a type. Names are in all caps, using underscores to seperate words.
    * Shadowing: You can declare a new variable with the same name as a previous variable. The new variable shadows the previous variable. (you have to use `let` again, and this is a redeffinition, not a mutation, the type can be completely different)
* Data Types: Rust is statically typed, but can infer types.
    * Scalar types: i32, u32, f32, f64, bool, char 
    * Compound types: tuples (fixed size, can have different types), arrays (*fixed* size, all elements must be the same type). Array size is part of the type signature, e.g. `[i32; 5]` is an array of 5 i32s.
* Functions
* Comments
* Control Flow
 