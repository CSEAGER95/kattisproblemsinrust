use std::io::{self, Write};

fn main() {
    let mut v = Vec::new();
    let mut ans = Vec::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    // Read 12 strings into the vector
    for _ in 0..12 {
        let mut s = String::new();
        handle.read_line(&mut s).unwrap();
        v.push(s.trim().to_string());
    }
    // Print the results
    if ans.is_empty() {
        println!("no sets");
    } else {
        for s in ans {
            println!("{}", s);
        }
    }
}
/*
The input to your program will consist of lines, 
each containing strings representing cards, each 
consisting of four characters ABCD where
A = 123 corresponding to the number of symbols.

B = DSO corresponding to diamonds (D), squiggles (S), and ovals (O).

C = STO corresponding to solid (S), striped (T), and open (O) fill styles.

D = RGP corresponding to red (R), green (G), and purple (P).

Think of the cards as being arranged in the input as follows:

+----------+
|  1  2  3 |
|  4  5  6 |
|  7  8  9 |
| 10 11 12 |
+----------+

Output

Output all sets you can find, one per line. For each set, 
output the numbers of the card in the set in sorted order. 
The sets should be listed in sorted order using the number of 
their first card, breaking ties using the numbers of the second
and third card in the set.

If no sets can be formed, output “no sets”. 
(Do not include any punctuation.)

The sample input/output corresponds to the illustration.

sample input 1:
3DTG 3DOP 2DSG
1SOP 1DTG 2OTR
3DOR 3STG 2DSP
3SSP 3OTG 1DTP


sample output 1:
1 8 11
2 9 12
3 7 12
5 7 9
6 8 12
7 10 11

*/