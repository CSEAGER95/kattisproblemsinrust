use std::io;

fn main() {
	let mut qaly = 0.0;
	let mut input = String::new();
		io::stdin().read_line(&mut input).expect("err");
		let numbers: usize = input.trim().parse().expect(" ");

		for _ in 0..numbers {
			input.clear();
			io::stdin().read_line(&mut input).unwrap();

			let parts: Vec<f64> = input.trim().split_whitespace().map(|x| x.parse::<f64>().unwrap()).collect();

			qaly+= parts[0] * parts[1];
			}
	println!("{:.4}", qaly);

	}
