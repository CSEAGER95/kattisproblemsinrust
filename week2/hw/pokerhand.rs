use std::io;
use std::collections::HashMap;
fn main(){
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();

	let hands: Vec<&str> = input.split_whitespace().collect();
	
	let mut counts: HashMap<char, i32> = HashMap::new();

	for hand in &hands {
		if let Some(first_char) = hand.chars().next() {
				*counts.entry(first_char).or_insert(0) += 1;
			}
		}
		if let Some((_most_frequent, max_count)) = counts.iter().max_by_key(|&(_, count)| count) {
			println!("{}", max_count);
		}
}
