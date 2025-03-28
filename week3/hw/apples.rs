use std::io::{self, BufRead};
fn main() {
    // Read dimensions
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();
    let dimensions = input.next().unwrap().unwrap();

    //grabs the first line and places them in a vector after parsing into integers.
    let nums: Vec<usize> = dimensions.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let height = nums[0];
    let width = nums[1];

    // Read the grid
    let mut grid = Vec::with_capacity(height);
    
    // stores the arrays
    let stdin = io::stdin();
    for line in input.take(height) {
        let row: Vec<char> = line.unwrap().trim().chars().collect();
        grid.push(row);
    }

    //looping through 2-D vectors, starting from the second from the bottom and moving up to 0
    //note .rev() function call.
    for i in (0..height-1).rev() {
        for j in 0..width {
            if grid[i][j] == 'a' {
                // Calculate final position in one pass
                let mut low = i;
                for k in (i+1)..height {
                    if grid[k][j] != '.' {
                        break;
                    }
                    low = k;
                }
                
                //apple fall
                if low != i {
                    grid[i][j] = '.';
                    grid[low][j] = 'a';
                }
            }
        }
    }
    // Print grid more efficiently
    for row in &grid {
        println!("{}", row.iter().collect::<String>());
    }
}