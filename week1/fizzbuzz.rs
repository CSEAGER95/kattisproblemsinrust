use std::io;

fn main() {
	let mut input = String::new();

	io::stdin().read_line(&mut input).expect("err");
	let numbers: Vec<i32> = input
		.trim()
		.split_whitespace()
		.map(|x| x.parse().expect(" "))
		.collect();
	
	let a = numbers[0];
	let b = numbers[1];
	let c = numbers[2];
	for i in 1..=c {
		if i % a == 0 && i % b == 0 {
			println!("FizzBuzz");
		}
		else if i % a == 0 {
			println!("Fizz");
		}
		else if i % b == 0 {
			println!("Buzz")
		}
		else {
			println!("{}", i);
		}
	}
}
