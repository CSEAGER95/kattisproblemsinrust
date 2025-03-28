use std::io;

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<usize> = input.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let total_spaces = parts[0];
    
    // Read divider positions
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let divider_positions: Vec<usize> = input.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Initialize possible_spaces
    let mut possible_spaces = vec![0; total_spaces];
    
    // Set the last element to 1 (equivalent to Python's possibleSpaces[-1] = 1)
    possible_spaces[total_spaces - 1] = 1;
    
    // Process each divider position
    for i in 0..divider_positions.len() {
        let divider_position = divider_positions[i];
        
        // Mark positions
        possible_spaces[divider_position - 1] = 1;
        possible_spaces[total_spaces - divider_position - 1] = 1;
        
        // Check differences between this position and all following positions
        for j in (i + 1)..divider_positions.len() {
            let diff = divider_positions[j] - divider_positions[i] - 1;
            if diff < total_spaces {
                possible_spaces[diff] = 1;
            }
        }
    }
    
    // Collect indices where value is 1
    let correct_spaces: Vec<String> = possible_spaces.iter()
        .enumerate()
        .filter(|&(_, &value)| value == 1)
        .map(|(i, _)| (i + 1).to_string())
        .collect();
    
    println!("{}", correct_spaces.join(" "));
}
