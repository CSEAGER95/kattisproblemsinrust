use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Trim newline and convert to vector of integers
    let input = input.trim();
    let mut intarr: Vec<i32> = input.chars()
        .map(|c| c.to_string().parse::<i32>().unwrap())
        .collect();
    
    // Clone the vector before sorting
    let intarr2 = intarr.clone();
    
    // Sort intarr
    intarr.sort();
    
    // Compare sorted intarr with unsorted intarr2
    if intarr == intarr2 {
        println!("0");
    } else {
        // This part is incomplete in the original code
        for m in 0..intarr2.len() {
            // Empty loop as in the original
        }
    }
}
