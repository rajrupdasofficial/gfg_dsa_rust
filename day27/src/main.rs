use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn merge_arrays(a: &mut Vec<i32>, b: &mut Vec<i32>) {
        let mut i = a.len() as i32 - 1;
        let mut j = 0;

        while i >= 0 && (j as usize) < b.len() {
            if a[i as usize] < b[j as usize] {
                break;
            } else {
                a.swap(i as usize, i as usize);
                b.swap(j as usize, j as usize);
                a.swap(i as usize, j as usize); // Actually swap elements between a[i] and b[j]
                std::mem::swap(&mut a[i as usize], &mut b[j as usize]);
                i -= 1;
                j += 1;
            }
        }

        a.sort();
        b.sort();
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        // Read first array line
        let arr1_line = lines.next().unwrap().unwrap();
        let mut a: Vec<i32> = arr1_line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        // Read second array line
        let arr2_line = lines.next().unwrap().unwrap();
        let mut b: Vec<i32> = arr2_line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        Solution::merge_arrays(&mut a, &mut b);

        for val in &a {
            print!("{} ", val);
        }
        println!();

        for val in &b {
            print!("{} ", val);
        }
        println!();

        println!("~");
    }
}
