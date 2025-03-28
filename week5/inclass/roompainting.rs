use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let parts: Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let n = parts[0] as usize; // number of paint can sizes
    let m = parts[1] as usize; // number of colours Joe needs
    
    // Read can sizes
    let mut can_sizes = Vec::with_capacity(n);
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    can_sizes = input.trim().split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    can_sizes.sort();
    
    // Read colors
    let mut colors = Vec::with_capacity(m);
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    colors = input.trim().split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    
    let mut min_waste: i64 = 0;
    
    // Binary search for each color
    for color in colors {
        let mut low = 0;
        let mut high = n - 1;
        
        while low <= high {
            let mid = low + (high - low) / 2;
            if can_sizes[mid] < color {
                low = mid + 1;
            } else {
                if mid == 0 {
                    break;
                }
                high = mid - 1;
            }
        }
        
        let waste = can_sizes[low] as i64 - color as i64;
        min_waste += waste;
    }
    
    println!("{}", min_waste);
}
