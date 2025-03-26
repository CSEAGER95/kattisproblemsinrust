use std::io;

fn main() {
    let mut max = 0;
    let mut winner = 0;
    
    for i in 1..=5 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let grades: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Not a valid number"))
            .collect();
        
        let sum: i32 = grades.iter().sum();
        
        if sum > max {
            max = sum;
            winner = i;
        }
    }
    
    println!("{} {}", winner, max);
}
