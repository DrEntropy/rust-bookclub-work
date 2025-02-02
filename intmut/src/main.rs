
use std::cell::RefCell;

fn main() {
     // interior mutability example
        let c = RefCell::new(String::from("Hello, "));
        // mutable borrow
        let mut b = c.borrow_mut();
        b.push_str(" World!");
        // immutable borrow not allowed here, even though b is not used again
        // println!("{}", c.borrow());
        // We can explicitly drop b though.
        drop(b);
        println!("{}", c.borrow());

         // normal mutable borrow
        let mut a = String::from("Hello, ");
        let  b = &mut a;
        b.push_str(" World!");

        // no drop needed here, compiler knows b is not used again
        println!("{}", a);
}
