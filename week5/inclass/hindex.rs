use std::io;
fn main() {
    let mut input = String::new();
    let mut v = Vec::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let n:usize = input.trim().parse().unwrap();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let num:i32 = input.trim().parse().unwrap();
        v.push(num);
    }
    v.sort();
    v.reverse();
    let buffer = (n/2) + 1;
    let size = n;
    let index = buffer;
    while buffer - v[index] < 0 {
        if buffer < v[index] {
            index-= 1;
            buffer+= 1;
        }
    }
    println!("{}",v[index]);
}
//1 2 3 5 6 7 8
// 1 2 5 6 7