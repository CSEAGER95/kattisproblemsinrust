use std::io;
use std::collections::HashSet;

fn is_leap_year(year: i32) -> bool {
    if year % 400 == 0 {
        return true;
    }
    if year % 100 == 0 {
        return false;
    }
    year % 4 == 0
}

fn is_valid_date(day: i32, month: i32, year: i32) -> bool {
    // Check basic ranges
    if month < 1 || month > 12 || day < 1 || year < 2000 {
        return false;
    }
    
    // Check days based on month
    let days_in_month = match month {
        2 => if is_leap_year(year) { 29 } else { 28 },
        4 | 6 | 9 | 11 => 30,
        _ => 31
    };
    
    day <= days_in_month
}

// Helper function to generate next permutation
fn next_permutation(arr: &mut [i32]) -> bool {
    let len = arr.len();
    let mut i = len - 1;
    
    while i > 0 && arr[i - 1] >= arr[i] {
        i -= 1;
    }
    
    if i == 0 {
        return false;
    }
    
    let mut j = len - 1;
    while arr[j] <= arr[i - 1] {
        j -= 1;
    }
    
    arr.swap(i - 1, j);
    arr[i..].reverse();
    true
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: i32 = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        // Extract only the digits from the input
        let mut digits: Vec<i32> = input
            .trim()
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        
        // Sort digits for permutation generation
        digits.sort();
        
        let mut valid_dates = HashSet::new();
        
        // Generate all permutations using a loop
        loop {
            // Check if current permutation forms a valid date
            let day = digits[0] * 10 + digits[1];
            let month = digits[2] * 10 + digits[3];
            let year = digits[4] * 1000 + digits[5] * 100 + digits[6] * 10 + digits[7];
            
            if is_valid_date(day, month, year) {
                valid_dates.insert((year, month, day));
            }
            
            // Generate next permutation
            if !next_permutation(&mut digits) {
                break;
            }
        }
        
        let mut dates_vec: Vec<(i32, i32, i32)> = valid_dates.into_iter().collect();
        
        if dates_vec.is_empty() {
            println!("0");
        } else {
            // Sort dates to find the earliest one
            dates_vec.sort_by(|a, b| {
                let (year_a, month_a, day_a) = a;
                let (year_b, month_b, day_b) = b;
                
                year_a.cmp(year_b)
                    .then(month_a.cmp(month_b))
                    .then(day_a.cmp(day_b))
            });
            
            let earliest = dates_vec[0];
            let (earliest_year, earliest_month, earliest_day) = earliest;
            
            println!("{} {:02} {:02} {:04}", dates_vec.len(), earliest_day, earliest_month, earliest_year);
        }
    }
}
