use std::io::{self, BufRead}
use std::collections::HashMap;
fn main() {
	let stdin = io::stdin(); //BufReader is faster
    let mut input = stdin.lock().lines;
    io::stdin().read_line(&mut input).unwrap();
    let mut min_dif: usize = input.trim().parse().unwrap();

    input.clear();
	io::stdin().read_line(&mut input).unwrap();
	let seats: Vec<&str> = input.split_whitespace().collect();

	//hashmap
	let mut map: HashMap<&str, usize> = HashMap::new();

    for (current_pos, &seat) in seats.iter().enumerate() {
        //adding each element and checking for repeats.
        if let Some(&previous_pos) = map.get(seat) {
            let distance = current_pos - previous_pos;
			//setting the minimum
            if distance < min_dif {
                min_dif = distance;
            }
        }
        
        //Similar to map.put(seat, currentPos) in Java, default way for setting positions if there are no repeats
        map.insert(seat, current_pos);
    }
    
    println!("{}", min_dif);
}