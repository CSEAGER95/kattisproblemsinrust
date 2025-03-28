extern crate rand;
use std::io;
use std::cmp::min;

fn main() {
    loop {
        println!("practice rust? press any key to receive a random practice problem or type 0 to finish");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        
        if user_input.trim() == "0" {
            println!("Exiting practice. Goodbye!");
            break;
        }
        
        let practice_number = rand::thread_rng().gen_range(1..=33);
        println!("Practice problem #{}", practice_number);
        
		match practice_number {
			1 => readMany(),
			2 => manyNums(),
			3 => firstLine(),
			4 => vectors(),
			5 => two_d_vectors(),
			6 => iterate(),
			7 => iterateWithIndex(),
			8 => hm(),
			9 => hs(),
			10 => bts(),
			11 => binary_search(),
			12 => sieve_of_erasthenes(),
			13 => next_permutation(),
			14 => character_operations(),
			15 => string_operations(),
			16 => searching_and_sorting(),
			17 => find_min_max(),
			18 => find_a_certain_value(),
			19 => random_num(),
			20 => numeric_conversions(),
			21 => min_max(),
			22 => absolute_value(),
			23 => math_constants(),
			24 => sqrt_pow(),
			25 => bitwise_operations(),
			26 => find_closest_value(),
			27 => binary_search_answer(),
			28 => dynamic_programming(),
			29 => sliding_window(),
			30 => dfs(),
			31 => bfs(),
			32 => recursive_with_memo(),
			33 => two_pointers_example(),
			_ => println!("Invalid option"),
		}
    }
}

fn checker(correct_answer: &str, typed_answer: &str) -> i32 {
    let correct_filtered: String = correct_answer.chars().filter(|c| !c.is_whitespace()).collect();
    let typed_filtered: String = typed_answer.chars().filter(|c| !c.is_whitespace()).collect();
    
    let mut diff_count = 0;
    let max_diff = correct_filtered.len() as i32;
    
    for (i, c1) in correct_filtered.chars().enumerate() {
        if i >= typed_filtered.len() || c1 != typed_filtered.chars().nth(i).unwrap() {
            diff_count += 1;
        }
    }
    
    // Cap the difference at the length of the correct answer
    min(diff_count, max_diff)
}

//DONE
fn readMany() {
    let mut total_diff = 0;
    
    println!("create a standard input object");
    let correct_answer1 = "let stdin = io::stdin();";
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("create a lines object to read multiple lines");
    let correct_answer2 = "let mut lines = stdin.lock().lines();";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff2 = checker(correct_answer2, &user_input);
    total_diff += diff2;
    
    println!("write a while loop to process each line. leave the code within brackets empty");
    let correct_answer3 = "while let Some(Ok(line)) = lines.next() { }";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff3 = checker(correct_answer3, &user_input);
    total_diff += diff3;
    
    println!("Total errors: {}", total_diff);
}

//DONE
fn manyNums() {
    let mut total_diff = 0;
    
    println!("parse multiple space-separated integers into a vector");
    let correct_answer1 = "let numbers: Vec<i32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();";
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("Total errors: {}", total_diff);
}

//DONE
fn firstLine() {
    let mut total_diff = 0;
    
    println!("parse the first line as test case count");
    let correct_answer1 = "let test_cases: usize = lines.next().unwrap().unwrap().parse().unwrap();";
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("write a for loop to process each test case leave the code in within the brackets (the processing part) blank");
    let correct_answer2 = "for _ in 0..test_cases { }";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff2 = checker(correct_answer2, &user_input);
    total_diff += diff2;
    
    println!("Total errors: {}", total_diff);
}

//data structures

//DONE
fn vectors() {
	let mut total_diff = 0;

	println!("instantiate a vector v");
	let correct_answer0 = "let mut v = Vec::new();";
	let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
	let diff0 = checker(correct_answer0, &user_input);
    total_diff += diff0;

	println!("write a vector 1-5 named 'v'");
    let correct_answer1 = "let v = vec![1,2,3,4,5];";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("access index 0 of vector v");
    let correct_answer2 = "let first = v[0];";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff2 = checker(correct_answer2, &user_input);
    total_diff += diff2;
    
    println!("use the get method to access index 0 of vector v");
    let correct_answer3 = "let maybe_first = v.get(0);";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff3 = checker(correct_answer3, &user_input);
    total_diff += diff3;
    
    println!("add the number 6 to vector v");
    let correct_answer4 = "v.push(6);";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff4 = checker(correct_answer4, &user_input);
    total_diff += diff4;
    
    println!("Total errors: {}", total_diff);
    if total_diff == 0 {
        println!("Perfect! All answers are correct.");
    }
}

//DONE
fn two_d_vectors() {
    let mut total_diff = 0;
    
    println!("create a 2D vector called 'grid' with predefined capacity height");
    let correct_answer1 = "let mut grid = Vec::with_capacity(height);";
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("populate the grid with rows of zeros of width length");
    let correct_answer2 = "for _ in 0..height { grid.push(vec![0;width]); }";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff2 = checker(correct_answer2, &user_input);
    total_diff += diff2;
    
    println!("Total errors: {}", total_diff);
}

//DONE
fn iterate() {
    let mut total_diff = 0;
    
    println!("write a for loop to iterate through a vector v. leave the brackets empty");
    let correct_answer1 = "for &num in &v {  }";
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("Total errors: {}", total_diff);
}

//DONE
fn iterateWithIndex() {
    let mut total_diff = 0;
    
    println!("write a for loop to iterate through vector v with index. leave the code within the brackets empty");
    let correct_answer1 = "for (i, &num) in v.iter().enumerate() { }";
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("Total errors: {}", total_diff);
}

//DONE
fn hm() {

	let mut total_diff = 0;
    println!("import HashMap from standard collections library");
	let correct_answer0 = "use std::collections::HashMap;";
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff0 = checker(correct_answer0, &user_input);
    total_diff += diff0;

    println!("create a new HashMap with String keys and i32 values");
    let correct_answer1 = "let mut map: HashMap<String, i32> = HashMap::new();";
	user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("insert a key-value pair");
    let correct_answer2 = "map.insert(\"key\".to_string(), 1);";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff2 = checker(correct_answer2, &user_input);
    total_diff += diff2;
    
    println!("increment a value or insert 0 if key doesn't exist");
    let correct_answer3 = "*map.entry(\"key\".to_string()).or_insert(0) += 1;";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff3 = checker(correct_answer3, &user_input);
    total_diff += diff3;

	println!("return the key of a given value in map");
    let correct_answer4 = "if let Some(&value) = map.get(\"key\") {	}";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff4 = checker(correct_answer4, &user_input);
    total_diff += diff4;

	println!("return the key of a given key in map");
    let correct_answer5 = "if map.contains_key(\"key\") { }";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff5 = checker(correct_answer5, &user_input);
    total_diff += diff5;

	println!("print out each key value pair like  key: -key- value: -value- in map");
    let correct_answer6 = "for (key, value) in &map {println!(\"key: {} value: {}\",key, value);}";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff6 = checker(correct_answer6, &user_input);
    total_diff += diff6;
    
    println!("Total errors: {}", total_diff);
}

//use std::collections::HashSet;

//DONE
fn hs() {
    let mut total_diff = 0;
    
    println!("import HashSet from standard collections library");
    let correct_answer0 = "use std::collections::HashSet;";
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff0 = checker(correct_answer0, &user_input);
    total_diff += diff0;
    
    println!("create a new HashSet for i32 values");
    let correct_answer1 = "let mut set: HashSet<i32> = HashSet::new();";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("insert a value into the set");
    let correct_answer2 = "set.insert(1);";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff2 = checker(correct_answer2, &user_input);
    total_diff += diff2;
    
    println!("check if a value exists in the set");
    let correct_answer3 = "if set.contains(&1) {";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff3 = checker(correct_answer3, &user_input);
    total_diff += diff3;
    
    println!("remove a value from the set");
    let correct_answer4 = "set.remove(&1);";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff4 = checker(correct_answer4, &user_input);
    total_diff += diff4;
    
    println!("iterate through all values in the set leave the code in the brackets empty");
    let correct_answer5 = "for value in &set { }";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff5 = checker(correct_answer5, &user_input);
    total_diff += diff5;
    
    println!("Total errors: {}", total_diff);
}

//DONE
fn bts() {
    let mut total_diff = 0;
    
    println!("import BTreeSet from standard collections library");
    let correct_answer0 = "use std::collections::BTreeSet;";
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff0 = checker(correct_answer0, &user_input);
    total_diff += diff0;
    
    println!("create a new BTreeSet for String values");
    let correct_answer1 = "let mut set: BTreeSet<String> = BTreeSet::new();";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("insert a string value into the set");
    let correct_answer2 = "set.insert(\"value\".to_string());";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff2 = checker(correct_answer2, &user_input);
    total_diff += diff2;
    
    println!("iterate through all values in the set leave the code in the bracket empty");
    let correct_answer3 = "for value in &set { }";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff3 = checker(correct_answer3, &user_input);
    total_diff += diff3;
    
    println!("Total errors: {}", total_diff);
}

//algorithms

//DONE
fn binary_search() {
    let mut total_diff = 0;
    
    println!("write the function signature for binary search");
    let correct_answer1 = "fn binary_search(arr: &[i32], target: i32) -> Option<usize> {";
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("initialize low and high indices");
    let correct_answer2 = "let mut low = 0; let mut high = arr.len() - 1;";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff2 = checker(correct_answer2, &user_input);
    total_diff += diff2;
    
    println!("write the main loop condition");
    let correct_answer3 = "while low <= high {";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff3 = checker(correct_answer3, &user_input);
    total_diff += diff3;
    
    println!("calculate the middle index");
    let correct_answer4 = "let mid = low + (high - low) / 2;";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff4 = checker(correct_answer4, &user_input);
    total_diff += diff4;
    
    println!("check if target is found");
    let correct_answer5 = "if arr[mid] == target { return Some(mid); }";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff5 = checker(correct_answer5, &user_input);
    total_diff += diff5;
    
    println!("handle when target is greater than mid element");
    let correct_answer6 = "else if arr[mid] < target { low = mid + 1; }";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff6 = checker(correct_answer6, &user_input);
    total_diff += diff6;
    
    println!("handle when target is less than mid element");
    let correct_answer7 = "else { if mid == 0 { break; } high = mid - 1; }";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff7 = checker(correct_answer7, &user_input);
    total_diff += diff7;
    
    println!("return None if not found");
    let correct_answer8 = "None";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff8 = checker(correct_answer8, &user_input);
    total_diff += diff8;
    
    println!("Total errors: {}", total_diff);
}

//DONE
fn sieve_of_erasthenes() {
    let mut total_diff = 0;
    
    println!("write the function signature");
    let correct_answer1 = "fn sieve_of_erasthenes(n: usize) -> Vec<usize> {";
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("initialize a boolean vector for prime flags");
    let correct_answer2 = "let mut is_prime = vec![true; n + 1];";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff2 = checker(correct_answer2, &user_input);
    total_diff += diff2;
    
    println!("mark 0 and 1 as not prime");
    let correct_answer3 = "is_prime[0] = false; is_prime[1] = false;";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff3 = checker(correct_answer3, &user_input);
    total_diff += diff3;
    
    println!("write the main sieve loop up to sqrt(n)");
    let correct_answer4 = "for i in 2..=(n as f64).sqrt() as usize {";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff4 = checker(correct_answer4, &user_input);
    total_diff += diff4;
    
    println!("check if number is prime");
    let correct_answer5 = "if is_prime[i] {";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff5 = checker(correct_answer5, &user_input);
    total_diff += diff5;
    
    println!("mark multiples as not prime");
    let correct_answer6 = "for j in (i*i..=n).step_by(i) { is_prime[j] = false; }";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff6 = checker(correct_answer6, &user_input);
    total_diff += diff6;
    
    println!("collect the prime numbers");
    let correct_answer7 = "is_prime.iter().enumerate().filter(|&(_, &is_p)| is_p).map(|(i, _)| i).collect()";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff7 = checker(correct_answer7, &user_input);
    total_diff += diff7;
    
    println!("Total errors: {}", total_diff);
}

//DONE
fn next_permutation() {
    let mut total_diff = 0;
    
    println!("write the function signature");
    let correct_answer1 = "fn next_permutation(arr: &mut [i32]) -> bool {";
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("find index k where arr[k] < arr[k+1]");
    let correct_answer2 = "let mut k = arr.len() - 2; while k >= 0 && arr[k] >= arr[k+1] {";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff2 = checker(correct_answer2, &user_input);
    total_diff += diff2;
    
    println!("check if no such index exists");
    let correct_answer3 = "if k == 0 { return false; } k -= 1; }";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff3 = checker(correct_answer3, &user_input);
    total_diff += diff3;
    
    println!("find index l where arr[l] > arr[k]");
    let correct_answer4 = "let mut l = arr.len() - 1; while arr[l] <= arr[k] { l -= 1; }";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff4 = checker(correct_answer4, &user_input);
    total_diff += diff4;
    
    println!("swap elements at indices k and l");
    let correct_answer5 = "arr.swap(k, l);";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff5 = checker(correct_answer5, &user_input);
    total_diff += diff5;
    
    println!("reverse the subarray after index k");
    let correct_answer6 = "arr[k+1..].reverse();";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff6 = checker(correct_answer6, &user_input);
    total_diff += diff6;
    
    println!("return true to indicate success");
    let correct_answer7 = "true";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff7 = checker(correct_answer7, &user_input);
    total_diff += diff7;
    
    println!("Total errors: {}", total_diff);
}

//string manipulation

//DONE
fn character_operations() {
    let mut total_diff = 0;
    let mut user_input = String::new();
    
    println!("check if input is a valid hex digit");
    let correct_answer1 = "let is_digit = input.is_digit(16);";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("check if input is uppercase");
    let correct_answer2 = "let is_upper = input.is_uppercase();";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff2 = checker(correct_answer2, &user_input);
    total_diff += diff2;
    
    println!("convert character to string");
    let correct_answer3 = "let char_to_string = input.to_string();";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff3 = checker(correct_answer3, &user_input);
    total_diff += diff3;
    
    println!("convert character to digit");
    let correct_answer4 = "let char_to_digit = input.to_digit(16).unwrap() as i32;";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff4 = checker(correct_answer4, &user_input);
    total_diff += diff4;
    
    println!("Total errors: {}", total_diff);
}

//DONE
fn string_operations() {
    let mut total_diff = 0;
    let mut user_input = String::new();
    
    println!("define a string variable S that contains \"lets go\"");
    let correct_answer1 = "let s = \"lets go\";";
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("split string by a delimiter into parts");
    let correct_answer2 = "let parts: Vec<&str> = s.split('-').collect();";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff2 = checker(correct_answer2, &user_input);
    total_diff += diff2;
    
    println!("remove a suffix from the string");
    let correct_answer3 = "let without_suffix = s.strip_suffix(\" go\").unwrap_or(\"\");";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff3 = checker(correct_answer3, &user_input);
    total_diff += diff3;
    
    println!("check if a string contains a character");
    let correct_answer4 = "if parts[0].contains('L') { true; }";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff4 = checker(correct_answer4, &user_input);
    total_diff += diff4;
    
    println!("convert string to uppercase");
    let correct_answer5 = "let upper = s.to_uppercase();";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff5 = checker(correct_answer5, &user_input);
    total_diff += diff5;
    
    println!("convert string to lowercase");
    let correct_answer6 = "let lower = s.to_lowercase();";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff6 = checker(correct_answer6, &user_input);
    total_diff += diff6;
    
    println!("convert string to vector of characters");
    let correct_answer7 = "let chars: Vec<char> = s.chars().collect();";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff7 = checker(correct_answer7, &user_input);
    total_diff += diff7;
    
    println!("convert vector of characters back to string");
    let correct_answer8 = "let s: String = chars.iter().collect();";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff8 = checker(correct_answer8, &user_input);
    total_diff += diff8;
    
    println!("join string parts with a delimiter");
    let correct_answer9 = "let joined = parts.join(\" \");";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff9 = checker(correct_answer9, &user_input);
    total_diff += diff9;
    
    println!("get a substring");
    let correct_answer10 = "let substring = &s[3..6];";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff10 = checker(correct_answer10, &user_input);
    total_diff += diff10;
    
    println!("Total errors: {}", total_diff);
}
//searching and sorting

//DONE
fn searching_and_sorting() {
    let mut total_diff = 0;
    let mut user_input = String::new();
    
    println!("sort vector v in ascending order");
    let correct_answer1 = "v.sort();";
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("sort vector v in descending order using a closure");
    let correct_answer2 = "v.sort_by(|a,b| b.cmp(a));";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff2 = checker(correct_answer2, &user_input);
    total_diff += diff2;
    
    println!("reverse the order of elements in vector v");
    let correct_answer3 = "v.reverse();";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff3 = checker(correct_answer3, &user_input);
    total_diff += diff3;
    
    println!("sort vector v by custom comparison function");
    let correct_answer4 = "v.sort_by(|a,b| a.compare_ranks(b));";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff4 = checker(correct_answer4, &user_input);
    total_diff += diff4;
    
    println!("sort vector v by a specific key");
    let correct_answer5 = "v.sort_by_key(|item| item.key);";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff5 = checker(correct_answer5, &user_input);
    total_diff += diff5;
    
    println!("sort vector v with partial comparison");
    let correct_answer6 = "v.sort_by(|a,b| a.partial_cmp(b).unwrap());";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff6 = checker(correct_answer6, &user_input);
    total_diff += diff6;
    
    println!("Total errors: {}", total_diff);
}

//DONE
fn find_min_max() {
    let mut total_diff = 0;
    let mut user_input = String::new();
    
    println!("find minimum value in vector v");
    let correct_answer1 = "let min = *v.iter().min().unwrap();";
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("find maximum value in vector v");
    let correct_answer2 = "let max = *v.iter().max().unwrap();";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff2 = checker(correct_answer2, &user_input);
    total_diff += diff2;
    
    println!("find minimum value by a specific key");
    let correct_answer3 = "let min_by_key = v.iter().min_by_key(|&item| item.key).unwrap();";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff3 = checker(correct_answer3, &user_input);
    total_diff += diff3;
    
    println!("Total errors: {}", total_diff);
}

//DONE
fn find_a_certain_value() {
    let mut total_diff = 0;
    let mut user_input = String::new();
    
    println!("prompt user for a number");
    let correct_answer1 = "println!(\"input a number 1-9\");";
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("read user input into a string");
    let correct_answer2 = "let mut input = String::new(); io::stdin().read_line(&mut input).unwrap();";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff2 = checker(correct_answer2, &user_input);
    total_diff += diff2;
    
    println!("trim whitespace from input");
    let correct_answer3 = "let input = input.trim();";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff3 = checker(correct_answer3, &user_input);
    total_diff += diff3;
    
    println!("find indices of elements matching input");
    let correct_answer4 = "let indices: Vec<usize> = v.iter().enumerate().filter(|(_, &value)| value == input).map(|(i, _)| i).collect();";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff4 = checker(correct_answer4, &user_input);
    total_diff += diff4;
    
    println!("Total errors: {}", total_diff);
}
//mathamtical operation

//DONE
fn random_num() {
    let mut total_diff = 0;
    let mut user_input = String::new();
    
    println!("import Rng trait from rand library");
    let correct_answer1 = "use rand::Rng;";
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("generate a random number between 0 and 100");
    let correct_answer2 = "let random_num = rand::thread_rng().gen_range(0..100);";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff2 = checker(correct_answer2, &user_input);
    total_diff += diff2;
    
    println!("Total errors: {}", total_diff);
}

//DONE
fn numeric_conversions() {
    let mut total_diff = 0;
    let mut user_input = String::new();
    
    println!("parse the string 42 into an i32");
    let correct_answer1 = "let num: i32 = \"42\".parse().unwrap();";
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("parse the string 3.14 into an f64");
    let correct_answer2 = "let num: f64 = \"3.14\".parse().unwrap();";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff2 = checker(correct_answer2, &user_input);
    total_diff += diff2;
    
    println!("convert a float float_val to an integer int_val");
    let correct_answer3 = "let int_val = float_val as i32;";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff3 = checker(correct_answer3, &user_input);
    total_diff += diff3;
    
    println!("convert an integer int_val to a float float_val");
    let correct_answer4 = "let float_val = int_val as f64;";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff4 = checker(correct_answer4, &user_input);
    total_diff += diff4;
    
    println!("Total errors: {}", total_diff);
}

//DONE
fn min_max() {
    let mut total_diff = 0;
    let mut user_input = String::new();
    
    println!("import min and max functions");
    let correct_answer1 = "use std::cmp::{min,max};";
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("find minimum of two integers");
    let correct_answer2 = "let minimum = min(a,b);";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff2 = checker(correct_answer2, &user_input);
    total_diff += diff2;
    
    println!("find maximum of two integers");
    let correct_answer3 = "let maximum = max(a,b);";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff3 = checker(correct_answer3, &user_input);
    total_diff += diff3;
    
    println!("find minimum of two floats");
    let correct_answer4 = "let minimum = a.min(b);";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff4 = checker(correct_answer4, &user_input);
    total_diff += diff4;
    
    println!("find maximum of two floats");
    let correct_answer5 = "let maximum = a.max(b);";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff5 = checker(correct_answer5, &user_input);
    total_diff += diff5;
    
    println!("Total errors: {}", total_diff);
}


//DONE
fn absolute_value() {
    let mut total_diff = 0;
    let mut user_input = String::new();
    
    println!("calculate absolute value of difference between two numbers");
	println!("abs_val instantiated based on num (the 1st variable) and other");
    let correct_answer1 = "let abs_val = (num - other).abs();";
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("Total errors: {}", total_diff);
}

//DONE
fn math_constants() {
    let mut total_diff = 0;
    let mut user_input = String::new();
    
    println!("define PI constant");
    let correct_answer1 = "const PI: f64 = std::f64::consts::PI;";
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("define Euler's number constant");
    let correct_answer2 = "const E: f64 = std::f64::consts::E;";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff2 = checker(correct_answer2, &user_input);
    total_diff += diff2;
    
    println!("Total errors: {}", total_diff);
}

//DONE
fn sqrt_pow() {
    let mut total_diff = 0;
    let mut user_input = String::new();
    
    println!("calculate square root of n, save it as sqrt, and convert to usize");
    let correct_answer1 = "let sqrt = (n as f64).sqrt() as usize;";
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("calculate num to the power of 2, save it as power");
    let correct_answer2 = "let power = num.pow(2);";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff2 = checker(correct_answer2, &user_input);
    total_diff += diff2;
    
    println!("Total errors: {}", total_diff);
}


//DONE
fn bitwise_operations() {
    let mut total_diff = 0;
    let mut user_input = String::new();
    
    println!("calculate bitwise a AND b save it as result");
    let correct_answer3 = "let and_result = a & b;";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff3 = checker(correct_answer3, &user_input);
    total_diff += diff3;
    
    println!("calculate bitwise a OR b save it as or_result");
    let correct_answer4 = "let or_result = a | b;";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff4 = checker(correct_answer4, &user_input);
    total_diff += diff4;
    
    println!("calculate bitwise a XOR  b save it as xor_result");
    let correct_answer5 = "let xor_result = a ^ b;";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff5 = checker(correct_answer5, &user_input);
    total_diff += diff5;
    
    println!("calculate bitwise NOT a save it as not_result");
    let correct_answer6 = "let not_result = !a;";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff6 = checker(correct_answer6, &user_input);
    total_diff += diff6;
    
    println!("calculate left shift a by 1 save it as left_shift");
    let correct_answer7 = "let left_shift = a << 1;";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff7 = checker(correct_answer7, &user_input);
    total_diff += diff7;
    
    println!("calculate right shift  a by 1 save it as right_shift");
    let correct_answer8 = "let right_shift = a >> 1;";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff8 = checker(correct_answer8, &user_input);
    total_diff += diff8;
    
    println!("iterate through all subsets using bitmask");
    let correct_answer9 = "for mask in 0..(1 << n) {";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff9 = checker(correct_answer9, &user_input);
    total_diff += diff9;
    
    println!("check if bit i is set");
    let correct_answer10 = "for i in 0..n { if ((mask >> i) & 1) == 1 {";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff10 = checker(correct_answer10, &user_input);
    total_diff += diff10;
    
    println!("Total errors: {}", total_diff);
}

//common problem solving patterns
//NOT DONE
fn find_closest_value() {
    let mut total_diff = 0;
    let mut user_input = String::new();
    
    println!("initialize the closest value variable with the first array element");
    let correct_answer1 = "let mut closest = array[0];";
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("initialize the minimum difference variable");
    let correct_answer2 = "let mut min_diff = (target - closest).abs();";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff2 = checker(correct_answer2, &user_input);
    total_diff += diff2;
    
    println!("iterate through all values in the array");
    let correct_answer3 = "for &num in &array {";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff3 = checker(correct_answer3, &user_input);
    total_diff += diff3;
    
    println!("calculate the absolute difference between target and current number");
    let correct_answer4 = "let diff = (target - num).abs();";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff4 = checker(correct_answer4, &user_input);
    total_diff += diff4;
    
    println!("update closest if current difference is smaller");
    let correct_answer5 = "if diff < min_diff { min_diff = diff; closest = num; }";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff5 = checker(correct_answer5, &user_input);
    total_diff += diff5;
    
    println!("handle the case of equal differences");
    let correct_answer6 = "else if diff == min_diff && num < closest { closest = num; }";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff6 = checker(correct_answer6, &user_input);
    total_diff += diff6;
    
    println!("Total errors: {}", total_diff);
}

//NOT DONE
fn binary_search_answer() {
    let mut total_diff = 0;
    let mut user_input = String::new();
    
    println!("write the function signature");
    let correct_answer1 = "fn binary_search_answer(min: f64, max: f64, epsilon: f64) -> f64 {";
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("initialize low and high values");
    let correct_answer2 = "let mut low = min; let mut high = max;";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff2 = checker(correct_answer2, &user_input);
    total_diff += diff2;
    
    println!("write the binary search loop condition");
    let correct_answer3 = "while high - low > epsilon {";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff3 = checker(correct_answer3, &user_input);
    total_diff += diff3;
    
    println!("calculate the middle point");
    let correct_answer4 = "let mid = (low + high) / 2.0;";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff4 = checker(correct_answer4, &user_input);
    total_diff += diff4;
    
    println!("handle the condition check and update high value");
    let correct_answer5 = "if condition(mid) { high = mid; }";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff5 = checker(correct_answer5, &user_input);
    total_diff += diff5;
    
    println!("handle the else case and update low value");
    let correct_answer6 = "else { low = mid; }";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff6 = checker(correct_answer6, &user_input);
    total_diff += diff6;
    
    println!("return the result");
    let correct_answer7 = "(low + high) / 2.0";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff7 = checker(correct_answer7, &user_input);
    total_diff += diff7;
    
    println!("Total errors: {}", total_diff);
}

//NOT DONE
fn dynamic_programming() {
    let mut total_diff = 0;
    let mut user_input = String::new();
    
    println!("write the function signature");
    let correct_answer1 = "fn dynamic_programming(n: usize) -> u64 {";
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("initialize the dp array");
    let correct_answer2 = "let mut dp = vec![0; n + 1];";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff2 = checker(correct_answer2, &user_input);
    total_diff += diff2;
    
    println!("set base cases");
    let correct_answer3 = "dp[0] = 0; dp[1] = 1;";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff3 = checker(correct_answer3, &user_input);
    total_diff += diff3;
    
    println!("write the loop to fill the dp array");
    let correct_answer4 = "for i in 2..=n {";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff4 = checker(correct_answer4, &user_input);
    total_diff += diff4;
    
    println!("calculate each dp value");
    let correct_answer5 = "dp[i] = dp[i-1] + dp[i-2];";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff5 = checker(correct_answer5, &user_input);
    total_diff += diff5;
    
    println!("return the result");
    let correct_answer6 = "dp[n]";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff6 = checker(correct_answer6, &user_input);
    total_diff += diff6;
    
    println!("Total errors: {}", total_diff);
}

//NOT DONE
fn sliding_window() {
    let mut total_diff = 0;
    let mut user_input = String::new();
    
    println!("write the function signature");
    let correct_answer1 = "fn sliding_window(arr: &[i32], k: usize) -> i32 {";
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("initialize max_sum and current_sum");
    let correct_answer2 = "let mut max_sum = 0; let mut current_sum = 0;";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff2 = checker(correct_answer2, &user_input);
    total_diff += diff2;
    
    println!("calculate sum of first k elements");
    let correct_answer3 = "for i in 0..k { current_sum += arr[i]; }";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff3 = checker(correct_answer3, &user_input);
    total_diff += diff3;
    
    println!("set initial max_sum");
    let correct_answer4 = "max_sum = current_sum;";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff4 = checker(correct_answer4, &user_input);
    total_diff += diff4;
    
    println!("write the sliding window loop");
    let correct_answer5 = "for i in k..arr.len() {";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff5 = checker(correct_answer5, &user_input);
    total_diff += diff5;
    
    println!("update current_sum by adding new element and removing oldest");
    let correct_answer6 = "current_sum = current_sum + arr[i] - arr[i - k];";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff6 = checker(correct_answer6, &user_input);
    total_diff += diff6;
    
    println!("update max_sum if current_sum is larger");
    let correct_answer7 = "max_sum = max_sum.max(current_sum);";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff7 = checker(correct_answer7, &user_input);
    total_diff += diff7;
    
    println!("return max_sum");
    let correct_answer8 = "max_sum";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff8 = checker(correct_answer8, &user_input);
    total_diff += diff8;
    
    println!("Total errors: {}", total_diff);
}

//NOT DONE
fn dfs() {
    let mut total_diff = 0;
    let mut user_input = String::new();
    
    println!("write the function signature");
    let correct_answer1 = "fn dfs(graph: &Vec<Vec<usize>>, start: usize, visited: &mut Vec<bool>) {";
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("mark current node as visited");
    let correct_answer2 = "visited[start] = true;";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff2 = checker(correct_answer2, &user_input);
    total_diff += diff2;
    
    println!("iterate through neighbors");
    let correct_answer3 = "for &neighbor in &graph[start] {";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff3 = checker(correct_answer3, &user_input);
    total_diff += diff3;
    
    println!("recursively visit unvisited neighbors");
    let correct_answer4 = "if !visited[neighbor] { dfs(graph, neighbor, visited); }";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff4 = checker(correct_answer4, &user_input);
    total_diff += diff4;
    
    println!("Total errors: {}", total_diff);
}

//NOT DONE
fn bfs() {
    let mut total_diff = 0;
    let mut user_input = String::new();

    println!("import VecDeque from the standard collections library");
    let correct_answer0 = "use std::collections::VecDeque;";
    io::stdin().read_line(&mut user_input).unwrap();
    let diff0 = checker(correct_answer0, &user_input);
    total_diff += diff0;
    
    println!("write the function signature");
    let correct_answer1 = "fn bfs(graph: &Vec<Vec<usize>>, start: usize) -> Vec<usize> {";
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("initialize visited array");
    let correct_answer2 = "let mut visited = vec![false; graph.len()];";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff2 = checker(correct_answer2, &user_input);
    total_diff += diff2;
    
    println!("initialize queue");
    let correct_answer3 = "let mut queue = VecDeque::new();";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff3 = checker(correct_answer3, &user_input);
    total_diff += diff3;
    
    println!("initialize distance array");
    let correct_answer4 = "let mut distance = vec![usize::MAX; graph.len()];";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff4 = checker(correct_answer4, &user_input);
    total_diff += diff4;
    
    println!("mark start node as visited");
    let correct_answer5 = "visited[start] = true;";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff5 = checker(correct_answer5, &user_input);
    total_diff += diff5;
    
    println!("set distance of start node");
    let correct_answer6 = "distance[start] = 0;";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff6 = checker(correct_answer6, &user_input);
    total_diff += diff6;
    
    println!("add start node to queue");
    let correct_answer7 = "queue.push_back(start);";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff7 = checker(correct_answer7, &user_input);
    total_diff += diff7;
    
    println!("write the main BFS loop");
    let correct_answer8 = "while let Some(node) = queue.pop_front() {";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff8 = checker(correct_answer8, &user_input);
    total_diff += diff8;
    
    println!("iterate through neighbors");
    let correct_answer9 = "for &neighbor in &graph[node] {";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff9 = checker(correct_answer9, &user_input);
    total_diff += diff9;
    
    println!("check if neighbor is unvisited");
    let correct_answer10 = "if !visited[neighbor] {";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff10 = checker(correct_answer10, &user_input);
    total_diff += diff10;
    
    println!("mark neighbor as visited");
    let correct_answer11 = "visited[neighbor] = true;";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff11 = checker(correct_answer11, &user_input);
    total_diff += diff11;
    
    println!("update distance to neighbor");
    let correct_answer12 = "distance[neighbor] = distance[node] + 1;";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff12 = checker(correct_answer12, &user_input);
    total_diff += diff12;
    
    println!("add neighbor to queue");
    let correct_answer13 = "queue.push_back(neighbor);";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff13 = checker(correct_answer13, &user_input);
    total_diff += diff13;
    
    println!("return the distance array");
    let correct_answer14 = "distance";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff14 = checker(correct_answer14, &user_input);
    total_diff += diff14;
    
    println!("Total errors: {}", total_diff);
}

//NOT DONE
fn recursive_with_memo() {
    let mut total_diff = 0;
    let mut user_input = String::new();
    
    println!("write the function signature");
    let correct_answer1 = "fn recursive_with_memo(n: usize, memo: &mut Vec<Option<u64>>) -> u64 {";
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("check if result is already memoized");
    let correct_answer2 = "if let Some(result) = memo[n] { return result; }";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff2 = checker(correct_answer2, &user_input);
    total_diff += diff2;
    
    println!("handle base cases");
    let correct_answer3 = "if n <= 1 { memo[n] = Some(n as u64); return n as u64; }";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff3 = checker(correct_answer3, &user_input);
    total_diff += diff3;
    
    println!("calculate result recursively");
    let correct_answer4 = "let result = recursive_with_memo(n-1, memo) + recursive_with_memo(n-2, memo);";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff4 = checker(correct_answer4, &user_input);
    total_diff += diff4;
    
    println!("memoize the result");
    let correct_answer5 = "memo[n] = Some(result);";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff5 = checker(correct_answer5, &user_input);
    total_diff += diff5;
    
    println!("return the result");
    let correct_answer6 = "result";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff6 = checker(correct_answer6, &user_input);
    total_diff += diff6;
    
    println!("Total errors: {}", total_diff);
}

//NOT DONE
fn two_pointers_example() {
    let mut total_diff = 0;
    let mut user_input = String::new();
    
    println!("write the function signature");
    let correct_answer1 = "fn two_pointers_example(arr: &[i32], target: i32) -> bool {";
    io::stdin().read_line(&mut user_input).unwrap();
    let diff1 = checker(correct_answer1, &user_input);
    total_diff += diff1;
    
    println!("initialize left pointer");
    let correct_answer2 = "let mut left = 0;";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff2 = checker(correct_answer2, &user_input);
    total_diff += diff2;
    
    println!("initialize right pointer");
    let correct_answer3 = "let mut right = arr.len() - 1;";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff3 = checker(correct_answer3, &user_input);
    total_diff += diff3;
    
    println!("write the main loop condition");
    let correct_answer4 = "while left < right {";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff4 = checker(correct_answer4, &user_input);
    total_diff += diff4;
    
    println!("calculate the current sum");
    let correct_answer5 = "let sum = arr[left] + arr[right];";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff5 = checker(correct_answer5, &user_input);
    total_diff += diff5;
    
    println!("check if sum equals target");
    let correct_answer6 = "if sum == target { return true; }";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff6 = checker(correct_answer6, &user_input);
    total_diff += diff6;
    
    println!("handle case when sum is less than target");
    let correct_answer7 = "else if sum < target { left += 1; }";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff7 = checker(correct_answer7, &user_input);
    total_diff += diff7;
    
    println!("handle case when sum is greater than target");
    let correct_answer8 = "else { right -= 1; }";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff8 = checker(correct_answer8, &user_input);
    total_diff += diff8;
    
    println!("return false if no pair is found");
    let correct_answer9 = "false";
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let diff9 = checker(correct_answer9, &user_input);
    total_diff += diff9;
    
    println!("Total errors: {}", total_diff);
}
