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
   

## Chapter 8

This chapter covered the standard collections in Rust:

- **Vectors**:
  - A vector is a dynamic array that can grow or shrink in size.
  - Created with the `vec!` macro or the `Vec::new()` function.
  - Elements are accessed using indexing (panic on failure) or the `get` method.
  - Iteration can be done with a `for` loop or the `iter` method.
  - Elements can be added with the `push` method or the `insert` method.
  - WHen a vector goes out of scope, its elements are dropped too.

- **Strings**:
    - Strings are utf-8 encoded. 
    - "string" means either `String` (heap allocated) or string slice `&str` (reference to part of a string)
    - The `String::from` function converts a string literal to a `String`.
    - The `to_string` method can also be used to convert a string slice to a `String`.
    - Strings can be concatenated with the `+` operator or the `format!` macro.  
    - Strings cannot be indexed but can be sliced. THis is usually also not what is wanted. 
    - Instead, use bytes() or chars() to iterate over the string. `chars` are 32 bit unicode code points. `bytes` are 8 bit UTF-8 codes.

- **Hash Maps**:
    - A hash map is a collection of key-value pairs. You have to use `use std::collections::Hashmap`.
    - Created with the `HashMap` type and `new` method.
    - Elements are added with the `insert` method. This will overwrite the value if the key already exists.
    - Values are accessed with the `get` method, which returns an `Option`.
    - The `entry` method allows for conditional insertion. 
        - It returns an `Entry` type.
        - `or_insert` method on `Entry` inserts a value if the key does not exist, and also returns a mutable reference to the value.
    - Elements can be removed with the `remove` method.
    - Iteration over keys and values can be done with for loops.


## Chapter 9

This chapter covered error handling in Rust. There are no exceptions, instead Rust uses the `Result` enum. It has two variants: `Ok` and `Err`.

Several helper functions were discussed:

    - `unwrap` returns the value in `Ok` or panics. Best for debugging and prototyping.
    - `expect` is like `unwrap` but allows you to specify the panic message.
    - `match` is the most common way to handle `Result`. It allows you to handle both `Ok` and `Err` cases explicitly.
    - ? operator is a shortcut for propagating errors. It can only be used in functions that return `Result` or `Option`.

There was also a discussion of when to `panic!` and when to return a `Result`. The general rule is to return a `Result` when the error is expected and can be handled by the caller. Use `panic!` when the error is unexpected and cannot be handled or leaves the program in an inconsistent state.


## Chapter 10: Generics, Traits, and Lifetimes

This chapter covered generics, traits, and lifetimes in Rust.

### **Generics**:

- In the same way that function can take parameters instead of discrete values, generics allow functions to take types as parameters.
- This allows for code reuse across different types. 
- The type parameter is defined in angle brackets after the function name, and can be then used as a type in the function definition.
- Type parameters can also be used for structs, enums, and methods.


Example struct:
```rust
struct Point<T> {
    x: T,
    y: T,
}
```

Note that all this says about point is that whatever T is, both x and y are of type T.  We could define a more general point struct like this:
```rust
struct Point<T, U> {
    x: T,
    y: U,
}
```

Of course the canonical example of an enum is option:
```rust
enum Option<T> {
    Some(T),
    None,
}
```

For methods, the type parameter is defined after the `impl` keyword. 
```rust
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

What this says is that we are implementing for a type T, a method for a Point<T>.  We need the impl<T> so that the compiler knows that T is a type parameter and not a concrete type T.  For example this is perfectly legal:


```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```
 
Note also that the method itself could take additional type parameters. 

- Rust uses `mono-morphization` to generate code for each type that is used. This means that the code is as efficient as if it were written for each type.

### **Traits**


 - Motivating traits:

Example Function:
```rust
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

This example doesn't work because the `>` operator is not defined for all types. Note that in language like C++, this would be a run time error. (Well didnt c++ fix this though? Concepts?). In rust a type T has *no* capabilities.   We can fix this by adding a trait bound to the type parameter. 


- Traits define shared behavior across types.
- Then can be used to constrain the types that a generic function can take. 
- Traits are similar to interfaces in other languages. 

Defining a trait:
```rust 
pub trait Summary {
    fn summarize(&self) -> String;
}
```
This says that any type implementing this trait must have a method called `summarize` that takes a reference to self and returns a string. Note that multiple functions can be defined in a trait.

Implementing a trait:
```rust
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```

- This is similar to implementing regular methods.  The difference is the inclusion of the trait name and `for`.

- Traits can have default implementations. This is done by adding code to the trait definition instead of the semicolon. Types can override the default implementation if they want.  But if they want to use the default implementation they can do it with an empty impl block:

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
impl Summary for NewsArticle {}
```

- Default implementations can call other methods in the same trait (even if they dont have default implementations). In this way, a trait can provide a lot of useful functionality and only require implementors to specify a small part of it.

- Note that it is not possible to call a default implementation from a method that overrides it. 

- **Traits as Parameters**:
    - Traits can be used as parameters in functions. This allows for more flexibility in the types that can be passed to a function. 
    - This is similar to using generics, but allows for more flexibility.

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

This is syntactic sugar for a generic function with *trait bounds*.  The above is equivalent to:

```rust 
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

- The full syntax allows for more complex trait bounds. For example, if we wanted to take two parameters that implement `Summary` we have:

```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}
```

But if we wanted to constrain the two parameters to have the same type we would have to use the full syntax:

```rust
pub fn notify<T: Summary,>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}
```

- To specify more then one trait bound, use the `+` operator. 

- `where` clauses can be used to make the function signature more readable when there are many trait bounds. The trait bounds are moved to the `where` clause. 


So this:
```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

becomes this:
```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

- Return types: 
   - You can use the `impl Trait` for a return type if you are returning a single type. 
   - This is useful when you want to return a type that implements a trait but you dont want to specify the exact type (for example Iterator types can be quite complicated?)
   - We will see in chapter 17 how to use trait objects to return multiple types.

- Using trait bounds to conditionally implement methods:
    - You can use trait bounds to conditionally implement methods. This is useful when you want to implement a method only for types that have a certain trait. 
    - This is done by using a `where` clause on the method implementation. 

- Conditionally implementing a trait for any type that implements another type is called a `blanket implementation`.

```rust
impl<T: Display> ToString for T {
    // --snip--
}
```



### Lifetime annotations

This is somehow the most confusing part of Rust.  Lifetimes are a way to ensure that references are valid for as long as they are used.

- Much like types, most of the time the compiler can infer lifetimes.
- We must use lifetime annotation when lifetimes of references could be related in different ways.
- Lifetime annotations are similar to... *NOTHING* in other languages of which I am aware. 
- Recall that lifetimes are a way to ensure that references are valid for as long as they are used. This prevents dangling references.


Example of where lifetime annotations are needed:
```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

This will not compile because the return value could be either `x` or `y` and the compiler cannot determine which.  We can fix this by adding a lifetime annotation:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

This says the return value will have the live as long as both `x` and `y`.  The lifetime annotation is placed after the function name and before the parameter list.  The lifetime name can be anything, but `'a` is common.  Then the lifetime annotations are placed after the `&` in the parameter list.

*This annotation shows how lifetimes are related, it doesnt CHANGE lifetimes.*  It says: This function, for some lifetime `'a`, takes two references with lifetime of at least `'a` and returns a reference with lifetime `'a`.  In practice this means that the return value will have the lifetime of the shorter of `x` and `y`.  This tells the rust borrow checker what it needs to know to *check* the lifetimes during compilation.


- There also exist lifetime bounds... not discussed in book but I saw it in an error message. For example, this is valid:
    
    ```rust
    fn longest<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    ```

- Lifetime annotations in struct defs

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

```

Annotations here tell the compiler that the struct cannot outlive the reference.  This is required for structs that contain references.

- Lifetime Elision
  - Rust has a growing list of rules for when lifetimes can be elided. 
  - Current rules for functions:
        - Each parameter that is a reference gets its own lifetime parameter.
        - If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
        - If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`, the lifetime of `self` is assigned to all output lifetime parameters.
  - If the compiler gets to the and of these rules and there are still lifetime parameters that have not been assigned, the compiler will error.
  - The third rule  means that methods don't need lifetime annotations in most cases.

- ``static` is a special lifetime that denotes that the reference *can* live for the entire duration of the program.  All string literals have the `static` lifetime.

- When using generics with lifetimes, the lifetime parameter must be declared before the type parameter. <`a, T>.   

## Chapter 11: Writing Automated Tests

- Rust has a built-in testing framework. 

- Functions marked with `#[test]` are run when you run `cargo test`.   Specific tests can be run with `cargo test test_name`.

- Tests output is normally hidden, but you can see it with `cargo test -- --show-output`.

- Unit tests (focused on smaller components in isolation) are placed in the same file as the code they are testing, in a module with the `#[cfg(test)]` attribute.  The `#[cfg(test)]` attribute tells the compiler to only compile the module when running tests.

- Integration tests are placed in a `tests` directory. Each file in the `tests` directory is a separate crate and uses the public API of the library.

- Test functions can: 

  - use `assert!` macro to test boolean expressions, as well as `assert_eq!` and `assert_ne!` for equality and inequality. A custom message can be added to the `assert!` macro.
  - use #[should_panic] attribute to test that a function panics.
  - use return type `Result<(), E>` . If it returns `Err(e)`, the test fails with message e, if it returns `Ok()`, the test passes.

- To facilitate testing, keep the 'main' code small and put most of the code in library crates that can be tested.

- I added some tests to the guessing game in the `guessing_game` folder for fun.

## Chapter 12: An I/O Project: Building a Command Line Program

This chapter covers building a command line program in Rust. The program is an very simplified implementation of the Unix 'grep' command.

This is a nice demonstration of the following concepts:

- Keeping the main function small and putting most of the code in library crates that can be tested.
- Using the `std::env` module to access command line arguments
- Reading from files
- Handling errors properly to give user helpful error messages
- Printing to stdout and stderr as appropriate
- Test Driven Development
- Using other rust concepts in context.

- The code is in the `minigrep` folder (final results, it might be modified further in future chapters)