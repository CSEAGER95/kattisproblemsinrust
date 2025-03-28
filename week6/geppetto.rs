use std::io;

fn main() {
    // Read N and M
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let n = parts[0]; // Number of ingredients
    let m = parts[1]; // Number of constraints
    
    // Read constraints
    let mut constraints = Vec::new();
    for _ in 0..m {
        let mut constraint_input = String::new();
        io::stdin().read_line(&mut constraint_input).expect("Failed to read line");
        let constraint: Vec<usize> = constraint_input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        constraints.push((constraint[0] - 1, constraint[1] - 1)); // Adjusting to 0-based indexing
    }
    
    // Count valid pizzas
    let mut count = 0;
    for mask in 0..(1 << n) {
        let mut valid = true;
        
        for &(i, j) in &constraints {
            // If both ingredients i and j are included, the pizza is invalid
            if ((mask >> i) & 1) == 1 && ((mask >> j) & 1) == 1 {
                valid = false;
                break;
            }
        }
        
        if valid {
            count += 1;
        }
    }
    
    println!("{}", count);
}
