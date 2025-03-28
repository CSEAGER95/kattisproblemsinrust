use std::io;
use std::collections::HashMap;

fn get_molecule_counts(from_molecule: &str, multiplier: i64) -> HashMap<String, i64> {
    let mut molecule_counts = HashMap::new();
    let mut molecule = String::new();
    let mut current_count = String::new();
    
    for c in from_molecule.chars() {
        if c.is_uppercase() {
            if !molecule.is_empty() {
                let count = if current_count.is_empty() { 1 } 
                          else { current_count.parse::<i64>().unwrap() };
                *molecule_counts.entry(molecule).or_insert(0) += count;
                current_count = String::new();
            }
            molecule = c.to_string();
        } else {
            current_count.push(c);
        }
    }
    
    // Process the last molecule
    if !molecule.is_empty() {
        let count = if current_count.is_empty() { 1 } 
                  else { current_count.parse::<i64>().unwrap() };
        *molecule_counts.entry(molecule).or_insert(0) += count;
    }
    
    // Apply multiplier
    for count in molecule_counts.values_mut() {
        *count *= multiplier;
    }
    
    molecule_counts
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let fline: Vec<&str> = input.trim().split_whitespace().collect();
    
    let orig_molecule = fline[0];
    let orig_molecule_count = fline[1].parse::<i64>().unwrap();
    
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let sline = input.trim();
    
    let original_molecule_counts = get_molecule_counts(orig_molecule, orig_molecule_count);
    let desired_molecule_counts = get_molecule_counts(&sline, 1);
    
    let mut min_max_count = 10000000000;
    
    for (key, &value) in &desired_molecule_counts {
        if !original_molecule_counts.contains_key(key) {
            min_max_count = 0;
            break;
        }
        
        let original_count = *original_molecule_counts.get(key).unwrap();
        min_max_count = min_max_count.min(original_count / value);
    }
    
    println!("{}", min_max_count);
}
