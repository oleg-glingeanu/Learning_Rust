// Cargos
use rand::Rng;

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

enum Direction {
    Up,
    Down,
    Left,
    Right,
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

    let go = Direction::Up;
    match go {
        Direction::Down => println!("go Down"),
        Direction::Left => println!("go Left"),
        Direction::Right => println!("go Right"),
        Direction::Up => println!("go Up"),
    }
    println!("");
    println!("");

    let mut rand = rand::thread_rng();

    let mut y = 0;

    while y < 10 {
        let mut go2 = Direction::Down;
        let a: i32 = rand.gen_range(0..4);

        match a {
            0 => go2 = Direction::Up,
            1 => go2 = Direction::Down,
            2 => go2 = Direction::Left,
            3 => go2 = Direction::Right,
        }

        match go2 {
            Direction::Down => println!("go Down"),
            Direction::Left => println!("go Left"),
            Direction::Right => println!("go Right"),
            Direction::Up => println!("go Up"),
        }

        y = y + 1;
    }
}
