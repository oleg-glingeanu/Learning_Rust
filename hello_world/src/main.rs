use std::io;
extern crate rand;
use rand::Rng;

fn main() {
    // Setting up the Starting Grid
    let plus_space = " + ";
    let minus_space = " - ";
    let straight_space = " | ";
    let grid: &mut [[String; 5]; 5] = &mut [
        [
            " 1 ".to_string(),
            straight_space.to_string(),
            " 2 ".to_string(),
            straight_space.to_string(),
            " 3 ".to_string(),
        ],
        [
            minus_space.to_string(),
            plus_space.to_string(),
            minus_space.to_string(),
            plus_space.to_string(),
            minus_space.to_string(),
        ],
        [
            " 4 ".to_string(),
            straight_space.to_string(),
            " 5 ".to_string(),
            straight_space.to_string(),
            " 6 ".to_string(),
        ],
        [
            minus_space.to_string(),
            plus_space.to_string(),
            minus_space.to_string(),
            plus_space.to_string(),
            minus_space.to_string(),
        ],
        [
            " 7 ".to_string(),
            straight_space.to_string(),
            " 8 ".to_string(),
            straight_space.to_string(),
            " 9 ".to_string(),
        ],
    ];

    // True = X;
    // False = O;
    // User input
    println!("Do you want to be X's or O's? ");
    let mut user_choice: String = String::new();
    io::stdin()
        .read_line(&mut user_choice)
        .expect("Failed to read your input");

    // Making user input a bool
    let mut choice: bool = true;
    if user_choice.trim() == "O" {
        choice = false;
    }
    println!("You chose {:?}", user_choice.trim());
    let mut game_finsihed = false;
    let counter = 5;
    while game_finsihed == false {
        // Where to place the users X or O
        println!("Where do you want to place your {}", user_choice.trim());
        printgrid(grid);
        let mut placement: String = String::new();
        io::stdin()
            .read_line(&mut placement)
            .expect("Failed to read your input");
        let placement_int: i32 = placement.trim().parse().unwrap();
        match placement_int {
            1 => plot_choice(grid, placement_int, choice),
            2 => plot_choice(grid, placement_int, choice),
            3 => plot_choice(grid, placement_int, choice),
            4 => plot_choice(grid, placement_int, choice),
            5 => plot_choice(grid, placement_int, choice),
            6 => plot_choice(grid, placement_int, choice),
            7 => plot_choice(grid, placement_int, choice),
            8 => plot_choice(grid, placement_int, choice),
            9 => plot_choice(grid, placement_int, choice),
            _ => println!("Enter again"),
        }
        let mut computer_choice: i32 = rand::thread_rng().gen_range(1..9);
        if computer_choice == placement_int {
            computer_choice = rand::thread_rng().gen_range(1..9);
        }
        let mut computer_ch = false;
        if choice == false {
            computer_ch = true;
        }
        plot_choice(grid, computer_choice, computer_ch)
    }
}

fn printgrid(grid: &mut [[String; 5]; 5]) {
    println!("================");
    // For Loop to print out initial State of the
    for row in grid.iter() {
        for cel in row {
            print!("{}", cel);
        }
        println!();
    }

    println!("================");
}

fn plot_choice(grid: &mut [[String; 5]; 5], choice: i32, ex_o: bool) {
    let user_choice_in_string = choice.to_string();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if user_choice_in_string == grid[i][j].trim() {
                if ex_o {
                    let plot: String = " X ".to_string();
                    grid[i][j] = plot;
                }
            }
        }
    }
    printgrid(grid);
}
