use std::io;
use rand::Rng;


pub fn run() {
    println!("Welcome to Rock-Paper-Scissors!");
    println!("Please choose rock, paper, or scissors.");

    loop {
        // Get player's choice
        let mut player_choice = String::new();
        io::stdin().read_line(&mut player_choice).expect("Failed to read line");
        let player_choice = player_choice.trim().to_lowercase();

        // Generate computer's choice
        let computer_choice = match rand::random::<u32>() % 3 {
            0 => "rock",
            1 => "paper",
            _ => "scissors",
        };

        // Determine winner
        let result = match player_choice.as_str() {
            "rock" if computer_choice == "scissors" => "You win!",
            "paper" if computer_choice == "rock" => "You win!",
            "scissors" if computer_choice == "paper" => "You win!",
            "rock" if computer_choice == "paper" => "You lose!",
            "paper" if computer_choice == "scissors" => "You lose!",
            "scissors" if computer_choice == "rock" => "You lose!",
            _ => "It's a tie!",
        };

        // Print result
        println!("You chose {}.", player_choice);
        println!("The computer chose {}.", computer_choice);
        println!("{}", result);

        // Ask to play again
        println!("Do you want to play again? (y/n)");
        let mut play_again = String::new();
        io::stdin().read_line(&mut play_again).expect("Failed to read line");
        if !play_again.trim().eq_ignore_ascii_case("y") {
            break;
        }
    }

    println!("Thanks for playing!");
}
