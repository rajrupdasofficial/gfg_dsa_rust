use std::io::{self, BufRead};

#[derive(Debug)]
struct Interval {
    start: i32,
    end: i32,
}

fn min_removal(intervals: &mut [Interval]) -> i32 {
    // Sort intervals by start time
    intervals.sort_by_key(|interval| interval.start);

    let mut count = 0;
    let mut prev_end = intervals[0].end;

    for interval in &intervals[1..] {
        if interval.start < prev_end {
            count += 1;
            if interval.end < prev_end {
                prev_end = interval.end;
            }
        } else {
            prev_end = interval.end;
        }
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read number of test cases
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        // Read number of intervals
        let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

        let mut intervals = Vec::with_capacity(n);

        for _ in 0..n {
            let line = lines.next().unwrap().unwrap();
            let parts: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            intervals.push(Interval {
                start: parts[0],
                end: parts[1],
            });
        }

        let result = min_removal(&mut intervals);
        println!("{}", result);
        println!("~");
    }
}
