use rand::Rng;
use std::{thread, time::Duration};

const NUM_LANES: usize = 4;
const FINISH_LINE: usize = 79; // Finish line at column 20

// Colors for each lane
const COLORS: [&str; NUM_LANES] = [
    "\x1b[31m", // Red
    "\x1b[32m", // Green
    "\x1b[33m", // Yellow
    "\x1b[34m", // Blue
];

fn main() {
    // Display the rat ASCII art and banner
    println!("
██████╗░░█████╗░████████╗██████╗░░█████╗░░█████╗░███████╗
██╔══██╗██╔══██╗╚══██╔══╝██╔══██╗██╔══██╗██╔══██╗██╔════╝
██████╔╝███████║░░░██║░░░██████╔╝███████║██║░░╚═╝█████╗░░
██╔══██╗██╔══██║░░░██║░░░██╔══██╗██╔══██║██║░░██╗██╔══╝░░
██║░░██║██║░░██║░░░██║░░░██║░░██║██║░░██║╚█████╔╝███████╗
╚═╝░░╚═╝╚═╝░░╚═╝░░░╚═╝░░░╚═╝░░╚═╝╚═╝░░╚═╝░╚════╝░╚══════╝
    ");

    println!("Welcome to the Rat Race!");
    println!("There are 4 lanes. Guess which one will reach the finish line first!");

    // Get player input for their guess
    let mut guess = String::new();
    println!("Enter your guess (1 to 4): ");
    std::io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess: usize = guess.trim().parse().expect("Please enter a valid number");

    // Check for valid input
    if guess < 1 || guess > NUM_LANES {
        println!("Invalid choice. Please enter a number between 1 and 4.");
        return;
    }

    // Simulate the race
    let mut rng = rand::thread_rng();
    let mut positions = vec![0; NUM_LANES]; // Track progress of each lane
    let mut winner = None;
    
    loop {
        // Move each lane's rat forward randomly
        for i in 0..NUM_LANES {
            positions[i] += rng.gen_range(1..=3); // Random speed between 1 and 3
        }

        // Check if any lane has reached the finish line
        for (i, &position) in positions.iter().enumerate() {
            if position >= FINISH_LINE { // Finish line at position 20
                winner = Some(i + 1); // Lanes are 1-indexed
                break;
            }
        }

        if winner.is_some() {
            break;
        }

        // Clear the screen (works on most systems)
        print!("\x1B[2J\x1B[H");

        // Draw the race lanes with the rats
        for i in 0..NUM_LANES {
            // Draw the lane with color
            let mut lane = vec!['-'; FINISH_LINE]; // The lane is represented with dashes
            if positions[i] < FINISH_LINE {
                lane[positions[i]] = '█'; // Place the "rat" at the current position
            } else {
                lane[FINISH_LINE - 1] = '█'; // If the rat has reached the finish line
            }
            let lane_str: String = lane.into_iter().collect();
            // Print the lane with its specific color
            println!("{}Lane {}: {}", COLORS[i], i + 1, lane_str);
        }

        // Wait a bit to simulate the race moving forward
        thread::sleep(Duration::from_millis(200));
    }

    // Announce the winner
    match winner {
        Some(winning_lane) => {
            println!("\nLane {} wins!", winning_lane);
            if winning_lane == guess {
                println!("Congratulations! You guessed correctly.");
            } else {
                println!("Sorry! You guessed wrong. Better luck next time.");
            }
        }
        None => println!("Something went wrong with the race."),
    }
}

