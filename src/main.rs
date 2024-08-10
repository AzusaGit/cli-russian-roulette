use colored::*;
use rand::prelude::*;
use std::io::{self, Write};

fn main() {
    println!("{}", "Russian Roulette Game\n".green());

    let mut points: i32 = 0;
    let bullets: i32 = 10;
    let mut bullet_count: i32 = bullets;
    let mut multiplier: i32 = 5;

    loop {
        if bullet_count == 1 {
            println!("{}", "Automatic reloading...".yellow());
            bullet_count = bullets;
        }

        let mut input = String::new();

        println!("Bullets remaining: {}", bullet_count);
        println!("{}", "Actions: ".yellow());
        println!("1. Pull the trigger");
        println!("2. Reload");
        println!("3. Exit game");
        print!("> ");
        io::stdout().flush().expect("Failed to flush");

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let input = input.trim();

        match input {
            "1" => {
                let mut rng = rand::thread_rng();
                let bullet_amount: i32 = rng.gen_range(1..=bullet_count);
                let lucky_num: i32 = rng.gen_range(1..=bullet_count);

                if bullet_amount != lucky_num {
                    if bullet_count <= bullets - 1 {
                        println!(
                            "\n{}",
                            format!("You survived! (+{}pts)", 10 + multiplier).green()
                        );
                        points = points + 10 + multiplier;
                        multiplier += 5;
                    } else {
                        println!("\n{}", "You survived! (+10pts)".green());
                        points += 10;
                        multiplier = 5;
                    }
                    bullet_count -= 1;
                } else {
                    println!("\n{}", "You died!".red());
                    println!("Total points: {}", points);
                    break;
                }
            }
            "2" => {
                println!("{}", "\nReloading...".green());
                bullet_count = bullets;
            }
            "3" => {
                println!("\nProgram exited");
                break;
            }
            _ => {
                println!("{}", "\nInvalid input. Try again".red());
            }
        }
    }
}
