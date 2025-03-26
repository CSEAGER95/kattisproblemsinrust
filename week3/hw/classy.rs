use std::io::{self, BufRead};
use std::cmp::Ordering;

// Struct to represent a person’s classification
#[derive(Debug, Eq, PartialEq)]
struct Classification {
    name: String,
    rank: Vec<i32>, // Store numeric ranks for easier comparison
}

impl Classification {
    // Custom comparator for sorting
    fn compare_ranks(&self, other: &Self) -> Ordering {
        // Compare ranks directly
        self.rank.cmp(&other.rank).reverse() // Reverse to sort descending
            .then_with(|| self.name.cmp(&other.name)) // If ranks are equal, sort alphabetically
    }
}

// Convert class string to numeric values
fn get_rank_value(rank: &str) -> i32 {
    match rank {
        "upper" => 3,
        "middle" => 2,
        "lower" => 1,
        _ => 2, // Default to "middle" (should never happen)
    }
}

// Parse a classification line into a `Classification` struct
fn parse_classification(line: &str) -> Classification {
    let parts: Vec<&str> = line.split(": ").collect();
    let name = parts[0].to_string();
    let rank_str = parts[1].strip_suffix(" class").unwrap_or("");

    let mut rank: Vec<i32> = rank_str.split('-').map(get_rank_value).collect();

    // Right-align ranks to a fixed length of 10 (padding with "middle")
    while rank.len() < 10 {
        rank.insert(0, 2); // Pad at the beginning with "middle" (value 2)
    }

    Classification { name, rank }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read number of test cases
    let m: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..m {
        // Read number of people in this case
        let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let mut group: Vec<Classification> = Vec::new();

        // Read each person's classification
        for _ in 0..n {
            if let Some(Ok(line)) = lines.next() {
                group.push(parse_classification(&line));
            }
        }

        // Sort using the custom comparison function
        group.sort_by(|a, b| a.compare_ranks(b));

        // Print sorted names
        for classification in &group {
            println!("{}", classification.name);
        }
        println!("==============================");
    }
}
