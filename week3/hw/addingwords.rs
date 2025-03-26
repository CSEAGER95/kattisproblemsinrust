use std::collections::HashMap;
use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut def: HashMap<String, i32> = HashMap::new();
    let mut ans: HashMap<i32, String> = HashMap::new();

    for line in stdin.lock().lines() { //stdin.lock() is very efficient and fast for reading inputs.
        let line = line.unwrap(); // reads line by line
        let mut iter = line.split_whitespace(); // creates an iterator over the line avoiding white spaces
        let command = iter.next(); //gets the first word either def calc or clear

        match command {
            Some("def") => {
                if let (Some(var), Some(value)) = (iter.next(), iter.next()) {
                    if let Ok(num) = value.parse::<i32>() {
                        if let Some(old_value) = def.get(var) {
                            ans.remove(old_value);
                        }
                        def.insert(var.to_string(), num);
                        ans.insert(num, var.to_string());
                    }
                }
            }
            Some("calc") => {
                let mut equation = String::new();
                let mut total = 0;
                let mut valid = true;
                let mut first = true;
                let mut sign = 1;

                while let Some(token) = iter.next() {
                    if token == "=" {
                        break;
                    }
                    if first {
                        first = false;
                        equation.push_str(token);
                        if let Some(&value) = def.get(token) {
                            total = value;
                        } else {
                            valid = false;
                        }
                    } else {
                        equation.push_str(&format!(" {}", token)); //formats strings and holds it in the equation variable.
                        if token == "+" {
                            sign = 1;
                        } else if token == "-" {
                            sign = -1;
                        } else {
                            if let Some(&value) = def.get(token) {
                                total += sign * value;
                            } else {
                                valid = false;
                            }
                        }
                    }
                }
                equation.push_str(" =");
                if valid && ans.contains_key(&total) {
                    writeln!(stdout, "{} {}", equation, ans[&total]).unwrap();
                } else {
                    writeln!(stdout, "{} unknown", equation).unwrap();
                }
                stdout.flush().unwrap(); //outputs to the terminal immediately
            }
            Some("clear") => {
                def.clear();
                ans.clear();
            }
            _ => {}
        }
    }
}
