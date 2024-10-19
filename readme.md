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
    
