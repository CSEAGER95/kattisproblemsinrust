use std::io;

fn isvalid(s: &str) -> bool {
    let mut has_l = false;
    let mut is_vowel = true;
    let mut cnt = 0;

    for c in s.chars() {
        if c == 'L' {
            has_l = true;
        }

        if "AEIOU".contains(c) {
            if is_vowel {
                cnt += 1;
            } else {
                is_vowel = true;
                cnt = 1;
            }
        } else {
            if is_vowel {
                is_vowel = false;
                cnt = 1;
            } else {
                cnt += 1;
            }
        }

        if cnt >= 3 {
            return false;
        }
    }

    has_l
}

fn solve(s: &mut Vec<char>, ways: &mut Vec<i64>, i: usize, ans: &mut i64) {
    if i == s.len() {
        if isvalid(&s.iter().collect::<String>()) {
            let mut count = 1;
            for (j, &w) in ways.iter().enumerate() {
                count *= w;
            }
            *ans += count;
        }
        return;
    }

    if s[i] != '_' {
        solve(s, ways, i + 1, ans);
    } else {
        // Try 'L'
        s[i] = 'L';
        solve(s, ways, i + 1, ans);

        // Try consonant 'B'
        s[i] = 'B';
        ways[i] = 20;
        solve(s, ways, i + 1, ans);

        // Try vowel 'A'
        s[i] = 'A';
        ways[i] = 5;
        solve(s, ways, i + 1, ans);

        // Reset
        s[i] = '_';
        ways[i] = 1;
    }
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s = s.trim().to_string();
    
    let mut s_vec: Vec<char> = s.chars().collect();
    let mut ways = vec![1; 101];
    let mut ans = 0;

    solve(&mut s_vec, &mut ways, 0, &mut ans);

    println!("{}", ans);
}


/*
Lea runs into a lot of words in her life. 
A lot of them she finds unpleasant. 
To compensate for that she started making up pleasant words. 
Lea makes up new words by writing a nice looking string of 
characters on a piece of paper. 
She than erases a few of the most nasty looking characters and 
replaces them with underscores ‘_’. 
After that she tries to replace the underscores with more 
acceptable characters trying to form a pleasant word.

Lea considers words pleasant if they do not contain
sequential vowels,

sequential consonants and contain at least one letter ‘L’.

In Croatian, the vowels are the letters A, E, I, O, and U only. 
All other letters are consonants.


Input

The first and only line of input contains a string of characters, 
at most
. The string contains only of uppercase English letters and ‘_’ 
characters. There will be at most

‘_’ characters.


Output

The first and only line of output should contain a single 
integer – the total number of pleasant words that can be formed
by substituting underscores with uppercase letters of the 
English alphabet.

sample input 1:
L_V

sample output 1:
5

sample input 2:
V__K

sample output 2:
10

sample input 3:
JA_BU_K_A

sample output 3:
485
*/