fn name() {
    print!("Oleg ");
}

fn s_name() {
    println!("Glingeanu");
}
// Basic arithmetic function
fn sub(a: i32, b: i32) -> i32 {
    return a - b;
}

// Main Programme
fn main() {
    println!("{}", sub(5, 3));
    name();
    s_name();
    let age = 12;

    // if statments
    if age >= 18 {
        println!("You are above 18");
    }
    // Else statement
    else {
        println!("You are below 18 ");
    }

    // match statement
    let some_int = 2;
    match some_int {
        1 => println!("its 1"),
        2 => println!("its 2"),
        3 => println!("its 3"),
        _ => println!("its another number"),
    }

    // Loop
    let mut i = 3;
    loop {
        println!("{:?}", i);
        i = i - 1;
        if i == 0 {
            break;
        }
    }

    // While Loop
    let mut x = 1;
    while x <= 5 {
        println!("{:?}", x);
        x = x + 1;
    }
}
