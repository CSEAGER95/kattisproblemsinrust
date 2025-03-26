use std::io;

fn main() {
    let inputs = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 22, 23, 25, 26, 28, 29, 33, 36, 39, 40, 44, 45, 46, 47, 48, 49, 50, 55, 56, 58, 59, 66, 69, 70, 77, 78, 79, 80, 88, 89, 99, 100, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 122, 123, 125, 126, 128, 129, 133, 136, 139, 140, 144, 145, 146, 147, 148, 149, 150, 155, 156, 158, 159, 166, 169, 170, 177, 178, 179, 180, 188, 189, 199, 200];
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let attempts: i32 = input
        .trim()
        .parse()
        .unwrap();
    for i in 0..attempts {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let buttons: i32 = input
            .trim()
            .parse()
            .unwrap();
            
        let mut closest = inputs[0];
        let mut min_diff = (buttons - closest).abs();
        
        for &num in &inputs {
            let diff = (buttons - num).abs();
            if diff < min_diff {
                min_diff = diff;
                closest = num;
            } else if diff == min_diff && num < closest {
                closest = num; // In case of tie, take the smaller value
            }
        }
        println!("{closest}")
    }
}