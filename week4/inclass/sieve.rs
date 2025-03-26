use std::time::Instant;

fn simple_sieve(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];
    let mut primes = Vec::new();

    for num in 2..=limit {
        if is_prime[num] {
            primes.push(num);
            let mut multiple = num * num;
            while multiple <= limit {
                is_prime[multiple] = false;
                multiple += num;
            }
        }
    }
    primes
}

fn segmented_sieve(n: usize) -> Vec<usize> {
    let limit = (n as f64).sqrt() as usize;
    let primes = simple_sieve(limit);
    let mut low = limit + 1;
    let mut high = 2 * limit;
    let mut prime_numbers = primes.clone();

    while low < n {
        if high > n {
            high = n;
        }

        let mut is_prime = vec![true; high - low + 1];

        for &prime in &primes {
            let mut start = (low / prime) * prime;
            if start < low {
                start += prime;
            }
            if start == prime {
                start += prime;
            }

            for j in (start..=high).step_by(prime) {
                is_prime[j - low] = false;
            }
        }

        for i in low..=high {
            if is_prime[i - low] {
                prime_numbers.push(i);
            }
        }

        low += limit;
        high += limit;
    }

    prime_numbers
}

fn main() {
    let start = Instant::now();
    let primes = segmented_sieve(1_000_000_000);
	let duration = start.elapsed();

    println!("Found {} primes under 1,000,000,000", primes.len());
    println!("Time taken: {:?}", duration);
}

