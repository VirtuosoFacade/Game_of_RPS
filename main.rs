// rock paper scissors game without cargo

use std::io;
// use rand::Rng;
use std::time::SystemTime;

fn main() {

    let mut player_sp = 0;
    let mut rustacean_sp = 0;
   // hates unused vars, only_snake_case

    let choices: [&str; 3] = ["rock", "paper", "scissors"];

    let mut player_input = String::new();

    println!("\nLets play rock paper scissors! 🎶🦀🎶");

    for n in 1..=3 {
        println!("\nRounds: {n}");

        println!("rock, paper, scissors.. go!");

        println!("Please enter your input:");

        // Clear the last input
        player_input.clear();

        // Read input from the user
        io::stdin()
            .read_line(&mut player_input)
            .expect("Failed to read line");

        let smoled_input = player_input.trim().to_lowercase();
        //println!("{}", smoled_input)

        /*let mut rng = rand::thread_rng();

        let rust_try = rng.gen_range(1..=3);*/

        let seed = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
        let mut rng: u128 = seed;

        // Simple linear congruential generator
        rng = (rng * 1103515245 + 12345) % 2_147_483_648;

        let rust_try = rng % 3 + 1;

        // player wins..

        if smoled_input == choices[0] && choices[rust_try as usize] == choices[2] {

            println!("Rustacean chose scissors. \nYou won.");

            player_sp += 1;
        }
        else if smoled_input == choices[1] && choices[rust_try as usize] == choices[0] {

            println!("Rustacean chose rock. \nYou won.");

            player_sp += 1;
        }
        else if smoled_input == choices[2] && choices[rust_try as usize] == choices[1] {

            println!("Rustacean chose paper. \nYou won.");

            player_sp += 1;
        }

        // Rustacean wins

        else if smoled_input == choices[2] && choices[rust_try as usize] == choices[0] {

            println!("Rustacean chose rock. \nRustacean won.");

            rustacean_sp += 1;
        }
        else if smoled_input == choices[0] && choices[rust_try as usize] == choices[1] {

            println!("Rustacean chose paper. \nRustacean won.");

            rustacean_sp += 1;
        }
        else if smoled_input == choices[1] && choices[rust_try as usize] == choices[2] {

            println!("Rustacean chose scissors. \nRustacean won.");

            rustacean_sp += 1;
        }

        // if tie happnes


        else if smoled_input == choices[rust_try as usize]
            {
                println!("oh! it was a tie..");
            }
        
        else {
                println!("you wanna play or not?");
            }

    }

    println!("\nFinal score:");
    println!("Player: {player_sp}");
    println!("Rustacean: {rustacean_sp}");

    if player_sp > rustacean_sp
    {
        println!("\nPlayer won the game!");
    }
    else if rustacean_sp > player_sp
    {
        println!("\nRustacean won the game!"); 
    }
    else if player_sp == rustacean_sp
    {
        println!("\nOh it was a tie!");
    }

}

//install rust if you want to use the main.exe
