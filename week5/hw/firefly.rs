use std::io;

fn main() {
    // Read n and h
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let n = parts[0]; // Length of the cave
    let h = parts[1]; // Height of the cave
    
    // Initialize an array to count obstacles at each height level
    let mut obstacles_at_height = vec![0; h+1]; // +1 because heights are 1-indexed
    
    // Read obstacle heights
    for i in 0..n {
        let mut obstacle_height = String::new();
        io::stdin().read_line(&mut obstacle_height).expect("Failed to read line");
        let obstacle_height: usize = obstacle_height.trim().parse().unwrap();
        
        // Determine if it's a stalagmite or stalactite
        let is_stalagmite = i % 2 == 0;
        
        if is_stalagmite {
            // Stalagmite grows from the floor
            for height in 1..=obstacle_height {
                obstacles_at_height[height] += 1;
            }
        } else {
            // Stalactite hangs from the ceiling
            for height in (h - obstacle_height + 1)..=h {
                obstacles_at_height[height] += 1;
            }
        }
    }
    
    // Find the minimum number of obstacles to destroy (skip index 0 as heights are 1-indexed)
    let min_obstacles = *obstacles_at_height.iter().skip(1).min().unwrap();
    
    // Count how many height levels achieve this minimum
    let count_min_levels = obstacles_at_height.iter().skip(1).filter(|&&count| count == min_obstacles).count();
    
    println!("{} {}", min_obstacles, count_min_levels);
}
