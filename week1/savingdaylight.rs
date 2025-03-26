use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split_whitespace().collect();
            
            if parts.len() == 5 {
                let (month, day, year) = (parts[0], parts[1], parts[2]);
                
                // Parse rise and set times
                let rise_time: Vec<i32> = parts[3].split(':')
                    .filter_map(|x| x.parse().ok())
                    .collect();
                let set_time: Vec<i32> = parts[4].split(':')
                    .filter_map(|x| x.parse().ok())
                    .collect();
                
                if rise_time.len() == 2 && set_time.len() == 2 {
                    // Calculate time difference
                    let mut hours = set_time[0] - rise_time[0];
                    let mut minutes = set_time[1] - rise_time[1];
                    
                    // Adjust for negative minutes
                    if minutes < 0 {
                        hours -= 1;
                        minutes += 60;
                    }
                    
                    println!("{} {} {} {} hours {} minutes", 
                        month, day, year, hours, minutes);
                }
            }
        }
    }
}