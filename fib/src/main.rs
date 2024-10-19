use std::env;
// Simple Fibonacci number generator

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: fib <number>");
        return;
    }
    let n: u64 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };
    match fib(n) {
        Some(fibn) => {
            println!("Fib {n} is {fibn}");
        }
        None => {
            println!("Overflow for {n}");
        }
    }
}

fn fib(n: u64) -> Option<u64> {
    if n == 0 {
        return Some(0);
    }
    if n == 1 {
        return Some(1);
    }
    // recursive version
    //fib(n - 1) + fib(n - 2)

    // iterative version, explicit loop
    // and checks for overflow
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 1..n {
        match a.checked_add(b) {
            Some(next) => {
                a = b;
                b = next;
            }
            None => {
                return None; // overflow
            }
        }
    }
    Some(b)
}

// Test cases
#[test]
fn test_fib() {
    assert_eq!(fib(0), Some(0));
    assert_eq!(fib(1), Some(1));
    assert_eq!(fib(2), Some(1));
    assert_eq!(fib(12), Some(144));
    assert_eq!(fib(30), Some(832040));
    assert_eq!(fib(100), None); // overflow
}
