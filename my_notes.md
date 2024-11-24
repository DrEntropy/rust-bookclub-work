# Rust Book Club

This file will capture lessons learned / etc from the Book and the Book club. 
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
   * Pointers are of two kinds: 
     * References (which do not own the data) 
     * Smart pointers like  Box, String, Vec (which do own the data).

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
 
 
- Structs
- Methods
- Other associated functions

### Defining Structs

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

- Structs are similar to tuples, but with named parts 
- Defines a new *type*

### Instantiating Structs

```rust
fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!("User1's email: {}",user1.email)
}
```

- Instantiate (create) by specifying values for each key
- To get values, use the `.` notation.  compare to R : `user1$email`
- To change values, the entire instance must be mutable.

### Using the Field Init Shorthand when Variables and Fields Have the Same Name
```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```
 
### Struct update syntax

```rust
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

- Creates a new `User` from an existing instance `user1` 
- Note that this *moves* data!
    - We can no longer use `user1` because we moved the username into `user2`
    - If we had also given a new username then `user1` would ok.

### Tuple Structs

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```
 
- Defines distinct types for `Color` and `Point`
- Access elements by destructuring 
- Alternately can use `.0` , `.1` etc. 

### Unit Structs

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

- Useful for cases where you need a type with a singleton value. (Placeholders or markers)
- More uses will be clearer when we discuss traits. 


### Ownership of Struct Data

- Examples so far used owned data (e.g. `String`)
- Ensures fields are valid as long as the struct is valid.
- Structs can store references, but this requires explicitly specifying *lifetimes* to ensure they remain valid (discussed in Chapter 10).


### #[derive(Debug)] for printing
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}");
}
```

### dbg! Macro

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // dbg! captures this intermediate value
        height: 50,
    };
    dbg!(&rect1);
}
```

* Takes ownership but then returns the value - print values inside an expression aids in debugging complex expressions. 
* Prints file and line number 
* Prints to `stderr` rather then `stdout`
 
### More traits
* [Appendix C](https://doc.rust-lang.org/stable/book/appendix-03-derivable-traits.html) has more derivable traits.
* More on traits in Chapter 10


### Defining Methods and Associated Functions

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { //implementaiton block
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
} // end impl block
```
 
* Methods are like functions but defined within the context of a struct, enum, or trait object using an `impl` block
* First parameter is always `self`. 
* Associated functions are defined in the `impl` block without `self` as the first parameter. 
* Methods are called like `rect1.area()` and associated functions are called like `Rectangle::square(3)`.
 

*`&self`, `&mut self`, `self`
    * `&self` is shorthand for `self : &Self`
    *  `Self` is shorthand for the object type. (`Rectangle`)
    * `&self` or `&mut self`, borrowing, is most common 
    * using just `self` and taking ownership is rare.
    

## Chapter 6 Enums and Pattern Matching

### Enums
- **Definition**: Enums allow you to define a type by enumerating its possible values.
  - Example:
    ```rust
    enum IpAddrKind {
        V4,
        V6,
    }
    ```
- **Usage**: Enums can store data alongside their variants.
  - Example:
    ```rust
    enum IpAddr {
        V4(String),
        V6(String),
    }
    ```
  - Another:
    ```rust
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    ```

### `Option` Enum
- A built-in Rust enum for handling null or absence of a value safely.
  - Example:
    ```rust
    enum Option<T> {
        Some(T),
        None,
    }
    ```

### Pattern Matching
- **`match` Expression**: A control flow construct for handling different enum variants.
  - Example:
    ```rust
    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
    Alabama,
    Alaska,
    // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {state:?}!");
                25
            }
        }
    } 
    ```
- `match` arms must cover all possible cases.
- The `_` placeholder matches any value without binding it.
- A variable can also be used as a binding on any value

### Conciseness with `if let`
- Provides a simpler way to match a single pattern.
  - Example:
    ```rust
    if let Some(value) = some_option {
        println!("Value is {}", value);
    }
    ```
- `if let` can be combined with `else` to handle the remaining cases.. 
- Syntactic sugar for `match` with one arm:
    - Example:
        ```rust
        match some_option {
            Some(value) => println!("Value is {}", value),
            None => (),
        }
        ```
    - Becomes:
        ```rust
        if let Some(value) = some_option {
            println!("Value is {}", value);
        }
        ```
        

### Key Takeaways
- Enums are flexible and can hold different types of data.
- Pattern matching is a powerful feature for handling complex logic.
- Rust enforces safety by ensuring all cases are handled in `match`.

Let me know if you need clarification on any part!

## Chapter 7



- **Packages and Crates**:
  - A *package* is a collection of one or more crates that provides a set of functionality.
  - A *crate* is a binary or library. The `Cargo.toml` file defines a package, which can contain multiple crates.
  - The `src` directory contains the source code for a crate, and the `main.rs` or `lib.rs` file is the crate root. 
  - See "Modules Cheat Sheet" for a reminder [here](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html#modules-cheat-sheet)

- **Modules**:
  - Modules allow you to organize code within a crate into groups for readability and reusability.
  - They control the scope and privacy of paths, enabling encapsulation.
  -  Code within a module is private from its parent modules by default. To make a module public, declare it with pub mod instead of mod. To make items within a public module public as well, use pub before their declarations.

- **Defining Modules**:
  - Use the `mod` keyword to define a module. The module's code is placed within curly braces *or* in a separate file.
  - Modules can be nested to create a hierarchy.

- **Paths for Referring to Items**:
  - *Absolute paths* start from the crate root, using the crate name or a literal `crate`.
  - *Relative paths* start from the current module, using `self`, `super`, or an identifier in the current scope.

- **Exposing Paths with `pub`**:
  - By default, items in a module are private.
  - Use the `pub` keyword to make items public and accessible from other modules.
  - Fields of a struct are private by default as well. The variants of an enum are all public if the enum is public.

- **Bringing Paths into Scope with `use`**:
  - The `use` keyword brings a path into scope, simplifying access to items.
  - It's commonly used to bring functions, types, or modules into the current scope.

- **Re-exporting with `pub use`**:
  - Combines `pub` and `use` to bring an item into scope and make it available for others to use.

- **Separation into Different Files**:
  - Modules can be moved to separate files to keep code organized.
  - The file structure should mirror the module hierarchy.


I created a simple example of module from chapter 7 (and adding doc strings from chapter 14) in `restaurant` folder.

## Chapter 14 

 Our book club did this chapter out of order because it was short are related to chapter 7.

 Chapter 14 covers: 
 - Release profiles
 - Crates.io  
 - Workspaces
 - Installing binaries from crates.io
 - Extending Cargo using custom commands

 - **Release Profiles**:
   - Cargo provides two default profiles: `dev` and `release`.
   - The `dev` profile is optimized for development, with debug symbols and fast compilation.
   - The `release` profile is optimized for release builds, with optimizations and no debug symbols.  (`cargo build --release`)
   - Profiles can be customized in the `Cargo.toml` file.  See [Cargo documentation](https://doc.rust-lang.org/cargo/reference/profiles.html) for more details.

 - **Documentation Comments**:
    - Documentation comments are written with `///` and support Markdown formatting.
    - Place these comments just before the item they are documenting.
    - If you include code examples in your documentation, you can run them as tests using `cargo test`! So these documentation comments always use `assert_eq!` and other such macros.
    - Documentation comments that start with `//!` are used to document the crate root or modules. They document the thing they are in.
    - Use `cargo doc --open` to generate and view documentation for your crate and dependencies.
    - Documentation comments are also used to generate documentation for crates published on crates.io.

 - **Crates.io**:
    - Crates.io is the official Rust package registry.
    - To publish a crate, use `cargo publish` after creating an account on crates.io. 
    - *ANYONE* can publish a crate, so be cautious when using crates from crates.io!   Name and typosquatting are a problem!
    - To use a crate, add it to the `Cargo.toml` file under `[dependencies]`.

- **Workspaces**:
    - A workspace is a collection of related packages that share the same `Cargo.lock` file.
    - To create a workspace, place multiple packages in a directory and create a `Cargo.toml` file at the workspace root.
    - For more details, see the [Cargo documentation](https://doc.rust-lang.org/cargo/reference/workspaces.html).

- **Installing Binaries from Crates.io**:
    - Cargo can install binaries from crates.io using `cargo install`.
    - The binary is installed in the `~/.cargo/bin` directory by default.
 
 - **Extending Cargo with Custom Commands**:
    - If a binary on your path is namesd `cargo-something`, you can run it as `cargo something`.  
    - This allows you to extend Cargo for custom tools. 
    - With this, cargo can be extended with custom commands using the `cargo install` command.
   