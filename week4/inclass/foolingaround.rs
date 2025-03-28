use std::io;
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num_cases: usize = input.trim().parse().unwrap();
    
    // First, precompute primes up to 1000
    let primes = get_primes(1000);
    let moves: Vec<usize> = primes.iter().map(|&p| p - 1).collect();
    
    for _ in 0..num_cases {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();
        
        // Calculate grundy number for n
        let mut grundy = vec![0; n + 1];
        for i in 1..=n {
            let mut mex_set = HashSet::new();
            for &m in &moves {
                if m <= i {
                    mex_set.insert(grundy[i - m]);
                }
            }
            
            // Find mex (minimum excludant)
            let mut mex = 0;
            while mex_set.contains(&mex) {
                mex += 1;
            }
            grundy[i] = mex;
        }
        
        if grundy[n] != 0 {
            println!("Alice");
        } else {
            println!("Bob");
        }
    }
}

fn get_primes(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    
    for i in 2..=(n as f64).sqrt() as usize {
        if is_prime[i] {
            for j in (i*i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    
    is_prime.iter().enumerate()
        .filter(|&(_, &is_p)| is_p)
        .map(|(i, _)| i)
        .collect()
}
