use std::io;
use std::collections::BTreeSet;

fn find_bracket_pairs(expr: &str) -> Vec<(usize, usize)> {
    let mut stack = Vec::new();
    let mut pairs = Vec::new();
    
    for (i, c) in expr.chars().enumerate() {
        if c == '(' {
            stack.push(i);
        } else if c == ')' {
            if let Some(start) = stack.pop() {
                pairs.push((start, i));
            }
        }
    }
    
    pairs
}

fn generate_expressions(expr: &str, pairs: &[(usize, usize)]) -> BTreeSet<String> {
    let mut result = BTreeSet::new();
    let n = pairs.len();
    
    // For each possible subset of pairs (2^n subsets)
    for mask in 1..(1 << n) {
        let mut chars: Vec<char> = expr.chars().collect();
        
        for i in 0..n {
            if (mask >> i) & 1 == 1 {
                let (start, end) = pairs[i];
                chars[start] = '\0';
                chars[end] = '\0';
            }
        }
        
        let new_expr: String = chars.into_iter().filter(|&c| c != '\0').collect();
        result.insert(new_expr);
    }
    
    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().to_string();
    
    let pairs = find_bracket_pairs(&input);
    let expressions = generate_expressions(&input, &pairs);
    
    // Print all expressions
    for expr in expressions {
        println!("{}", expr);
    }
}

