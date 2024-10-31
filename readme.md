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
    * Defined with `fn` keyword
    * snake case by convention
    * In function signatures, you must declare the type of each parameter and return value. This allows for type inference in the body of the function and elsewhere. You almost never have to define other variable types.
    * Function bodies are made up of a series of statements and an optional expression. "Rust is an expression-based language"
      * Statements are instructions that perform some action and do not return a value. They end in a semicolon.
      * Expressions evaluate to a resultant value. They do not end in a semicolon, and if you add one, you turn the expression into a statement.
    * Scope blocks are expressions. The last expression in the block is the return value.
    * Functions can return values with the `return` keyword, but it is idiomatic to omit it and just have the last expression in the block be the return value, except when returning early.  

* Comments
    * `//` for single line comments
    * `/* */` for multi-line comments, but ideomatic to use `//` for multi-line comments too, one on each line.
* Branching with `if`
    * If expressions are similar to c type languages, except the condition must evaluate to a `bool`.
    * `if` is an expression, so it can be used on the right side of a `let` statement!
* Loops
    * `loop` keyword creates an infinite loop. You can use `break` to exit the loop, and `continue` to skip the rest of the loop and start the next iteration. Loops are expressions too! To return a value from a loop, you can use the `break` keyword with the value you want to return.
    * loop labels are used to break out of nested loops. To create a label, put an identifier followed by a colon before the loop. To break out of a loop with a label, use `break 'labelname;` Loop labels must begin with a single quote. (Similar to lifetimes as we will see later)
    * `while` loops are similar to other languages.
    * `for` loops are provided to iterate over a collection: 
        ```rust
        let a = [10, 20, 30, 40, 50];
        for element in a {
            println!("the value is: {element}";
        }
        ```
    * The Rust standard library provides Range types that generate sequences of numbers to make loops easier and safer:
        ```rust
        for number in (1..4).rev() {
            println!("{}!", number);
        }
        println!("LIFTOFF!!!");
        ```
    
## Chapter 4
Understnading ownership 

### What is Ownership

* Heap vs Stack memory.   
   * Function parameters and local variables are stored on the stack.
   * Heap memory is used for dynamically sized data, and pointers to these values are stored on the stack and *own* the memory.

* Ownership rules:
  * When a non-copy trait value is assigned, passed, or returned, the ownership is transferred to the new variable. The old variable can no longer be used. 
  * Small stack-sized types (like integers) are copied instead of moved. They implement the Copy trait and do not own any heap memory. 
  * A value can can have only one owner at a time.
  * When the owner goes out of scope, the value is dropped.
 

### References and Borrowing

* References allow you to refer to a value without taking ownership of it. 
* References are created with the `&` operator.
* Mutable references are created with `&mut` 

* Rules of references:
    * You can have either one mutable reference or any number of immutable references to a value at the same time. This ensures safe, race-free access to data.
    * "At the same time" refers to the lifetime of the reference, not the entire scope in which it’s defined. A mutable reference's lifetime ends after its last use, making it eligible to "go out of scope" and allowing other references to take its place if needed. Note that the rust book is a bit vague on this and *seems* to conflate scope  and lifetime.
    * References must always be valid. Rust's borrow checker enforces that a reference cannot outlive the data it refers to, preventing dangling references and ensuring memory safety.

* The 'borrow checker' enforces rules at compile time to ensure that references are valid.

### Slices

Slices are special kinds of references because they are “fat” pointers, or pointers with metadata. 

There are two types of slices covered in this chapter: string slices and array slices.

* String slices are references to a portion of a string. They are created using the `&` operator. For example, `let s = String::from("hello"); let slice = &s[0..2];` creates a slice of the string `s` that contains the first two characters.   They have type `&str`.  Note string literals also have type `&str`. So functions taking refrences to strings should take `&str` to be generally useful.  (String refrences are automatically coerced to slices, which is trival since `&s[..]` is basically the same as `&s`).

* Array slices are similar to string slices, but they are references to a portion of an array. They are created using the same `&` operator. For example, `let a = [1, 2, 3, 4, 5]; let slice = &a[1..3];` creates a slice of the array `a` that contains the second and third elements. In this case the type is `&[i32]`.

### Alternate chapter 4

[Rust Book Experimental](https://rust-book.cs.brown.edu/) provides a interactive version of the book with quizes. Most of the book seems to be the same *except* for chapter 4, which has a different structure different content. It is worth reviwing as a supplement to the main book.

As this version states "A common theme will be understanding whether a function is actually safe or unsafe. Rust will always reject an unsafe program1. But sometimes, Rust will also reject a safe program. These case studies will show how to respond to errors in both situations."   Some of the quizes are quite challenging, especially those that ask you to imagine what code might not be undefined behavior if the compiler allowed it.  

The section on fixing ownership issues is also quite useful and worth bookmarking: 
[Fixing Ownership Issues](https://rust-book.cs.brown.edu/ch04-03-fixing-ownership-errors.html) 

This section has many examples of ownership issues, and illustrates not only how to fix them by why they are unsafe.  
 

## Chapter 5
This is my chapter... see seperate charts.