use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let a = parts[0] as usize;
    let d = parts[1] as usize;
    
    let mut asc = vec![[0.0, 0.0]; a];
    let mut desc = vec![[0.0, 0.0]; d];
    
    // Read ascending segments
    for i in 0..a {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let parts: Vec<f64> = input.trim().split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        asc[i][0] = parts[0]; // distance
        asc[i][1] = parts[1]; // time
    }
    
    // Read descending segments
    for i in 0..d {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let parts: Vec<f64> = input.trim().split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        desc[i][0] = parts[0]; // distance
        desc[i][1] = parts[1]; // time
    }
    
    // Calculate total distance and time
    let mut total_distance = 0.0;
    let mut total_time = 0.0;
    for i in 0..a {
        total_distance += asc[i][0];
        total_time += asc[i][1];
    }
    
    println!("{:.5}", ans(&asc, &desc, total_distance, total_time));
}

fn asct(asc: &[[f64; 2]], time: f64) -> f64 {
    let mut completed_time = 0.0;
    let mut distance = 0.0;
    
    for segment in asc {
        if completed_time < time && segment[1] + completed_time < time {
            distance += segment[0];
            completed_time += segment[1];
        } else {
            distance += (segment[0] / segment[1]) * (time - completed_time);
            return distance;
        }
    }
    
    distance
}

fn desct(desc: &[[f64; 2]], time: f64, total_distance: f64) -> f64 {
    let mut completed_time = 0.0;
    let mut distance = 0.0;
    
    for segment in desc {
        if completed_time < time && segment[1] + completed_time < time {
            distance += segment[0];
            completed_time += segment[1];
        } else {
            distance += (segment[0] / segment[1]) * (time - completed_time);
            return total_distance - distance;
        }
    }
    
    total_distance - distance
}

fn ans(asc: &[[f64; 2]], desc: &[[f64; 2]], total_distance: f64, total_time: f64) -> f64 {
    let mut min_time = 0.0;
    let mut max_time = total_time;
    let epsilon = 1e-6;
    
    while max_time - min_time > epsilon {
        let mid_time = (max_time + min_time) / 2.0;
        let asc_distance = asct(asc, mid_time);
        let desc_distance = desct(desc, mid_time, total_distance);
        
        if (asc_distance - desc_distance).abs() < epsilon {
            max_time = mid_time;
        } else if asc_distance < desc_distance {
            min_time = mid_time;
        } else {
            max_time = mid_time;
        }
    }
    
    (max_time + min_time) / 2.0
}
