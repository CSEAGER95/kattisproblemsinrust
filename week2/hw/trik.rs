use std::io;

fn main() {
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	let moves = input.trim();

	let mut cups = [false, true, false, true];

	for ch in moves.chars() {
		match ch {
			'A' => cups.swap(1,2),
			'B' => cups.swap(2,3),
			'C' => cups.swap(1,3),
			_=> {}
		}
	}
	for (i, &cup) in cups.iter().enumerate() {
		if cup{
			println!("{}", i);
			break;
		}
	}
}
