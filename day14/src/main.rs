use std::io::{self, BufRead};

fn my_atoi(s: &str) -> i32 {
    let mut ans = 0;
    let mut i = 0;
    let mut sign = 1;

    while i < s.len() && s.as_bytes()[i] == b' ' || s.as_bytes()[i] == b'\t' || s.as_bytes()[i] == b'\n' || s.as_bytes()[i] == b'\r' || s.as_bytes()[i] == b'\f' || s.as_bytes()[i] == b'\v' {
        i += 1;
    }

    if i < s.len() && (s.as_bytes()[i] == b'-' || s.as_bytes()[i] == b'+') {
        sign = if s.as_bytes()[i] == b'-' { -1 } else { 1 };
        i += 1;
    }


    while i < s.len() && s.as_bytes()[i] >= b'0' && s.as_bytes()[i] <= b'9' {

        if ans > i32::MAX / 10 || (ans == i32::MAX / 10 && (s.as_bytes()[i] - b'0') > (i32::MAX % 10) as u8) {
            return if sign == 1 { i32::MAX } else { i32::MIN };
        }
        ans = ans * 10 + (s.as_bytes()[i] - b'0') as i32;
        i += 1;
    }

    sign * ans
}

fn main() {

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();


    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {

        let line = lines.next().unwrap().unwrap();
        let ans = my_atoi(&line);
        println!("{}\n~", ans);
    }
}
