use std::cmp::{min, max};
use std::io::{self, BufRead};

const INF: i32 = 1_000_000_010;
const MOD: i32 = 1_000_000_007;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut lines = handle.lines();
    let n: i32 = lines.next().unwrap().unwrap().parse().unwrap();

    let mut hi = INF;
    let mut lo = 0;

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace();
        let l: i32 = nums.next().unwrap().parse().unwrap();
        let r: i32 = nums.next().unwrap().parse().unwrap();

        hi = min(hi, r);
        lo = max(lo, l);
    }

    if lo > hi {
        println!("edward is right");
    } else {
        println!("gunilla has a point");
    }
}
