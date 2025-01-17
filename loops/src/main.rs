fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    println!("labeled loops:");
    labeledloops();
    println!("blast off:");
    blast_off();
    println!("iterate:");
    iterate();
}

fn labeledloops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}



fn blast_off() {
    //let mut number = 3;

    //while number != 0 {
    //    println!("{number}!");
    //
    //    number -= 1;
    //}
    // safer and cleaner way
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    

    println!("LIFTOFF!!!");
}

fn iterate() {
    let a = [10, 20, 30, 40, 50];
    
    for element in a {
        println!("the value is: {element}");
    }
 
}