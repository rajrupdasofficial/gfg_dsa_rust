use std::io::{self, BufRead};

fn missing_number(arr: &mut Vec<i32>) -> i32 {
    let n = arr.len() as i32;
    for i in 0..n {
        while arr[i as usize] > 0
            && arr[i as usize] <= n
            && arr[i as usize] != arr[(arr[i as usize] - 1) as usize]
        {
            let temp = arr[i as usize];
            arr[i as usize] = arr[(temp - 1) as usize];
            arr[(temp - 1) as usize] = temp;
        }
    }
    for i in 0..n {
        if arr[i as usize] != i + 1 {
            return i + 1;
        }
    }
    n + 1
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut arr: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let res = missing_number(&mut arr);
        println!("{}", res);
        println!("~");
    }
}
