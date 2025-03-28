use std::io;
fn main() {
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	let numbers: usize = input.trim().parse().unwrap();
	for _ in 0..numbers {
		let mut input = String::new();
		io::stdin().read_line(&mut input).unwrap();
		let entries: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
		let a = entries[0];
		let b = entries[1];
		let c = entries[2];

		if a+b==c || a-b == c || b-a == c || a*b == c || a/b == c || b/a == c {
			println!("Possible");
		} else {
			println!("Impossible");
		}
	}
}
