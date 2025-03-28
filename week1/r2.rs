use std::io;

fn main() {
	let mut input = String::new();

	io::stdin().read_line(&mut input).expect(" ");
	let numbers: Vec<i32> = input
		.trim()
		.split_whitespace()
		.map(|x| x.parse().expect("Not a calid number"))
		.collect();

	let s = numbers[0];
	let r1 = numbers[1];
	println!("{}", ((r1 * 2) - s));
}
