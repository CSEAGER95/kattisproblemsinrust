use std::io::{self, BufRead};
fn main() {
    let mut times = Vec::new();
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    if let Some(Ok(line)) = lines.next() {
        times = line
            .split_whitespace()
            .filter_map(|s| s.parse::<f64>().ok())
            .collect();
    }
    times.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let target: f64 = match lines.next() {
        Some(Ok(line)) => match line.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Failed to parse target value.");
                return;
            }
        },
        _ => {
            eprintln!("Failed to read target value.");
            return;
        }
    };
    
    if (times[1] + times[2] + times[3]) / 3.0 <= target {
        println!("infinite")
    }
    else if (times[0] + times[1] + times[2]) / 3.0 > target {
        println!("impossible")
    }
    else {
        let out: f64 = (target * 3.0) - times[1] - times[2];
        println!("{:.2}", out);
    }
}
