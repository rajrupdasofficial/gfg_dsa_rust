use std::collections::HashMap;

struct Solution;

impl Solution {
    fn non_repeating_char(s: &str) -> char {
        let mut freq: HashMap<char, i32> = HashMap::new();

        for c in s.chars() {
            *freq.entry(c).or_insert(0) += 1;
        }

        for c in s.chars() {
            if *freq.get(&c).unwrap() == 1 {
                return c;
            }
        }
        '$'
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let t: i32 = input.trim().parse().expect("Please type a number!");

    for _ in 0..t {
        let mut s = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        s = input.trim().to_string();
        input.clear();

        let ans = Solution::non_repeating_char(&s);

        if ans != '$' {
            println!("{}", ans);
        } else {
            println!("-1");
        }

        println!("~");
    }
}
