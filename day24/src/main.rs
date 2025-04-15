use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Interval {
    start: i32,
    end: i32,
}

fn merge_overlap(mut intervals: Vec<Interval>) -> Vec<Interval> {
    if intervals.is_empty() {
        return vec![];
    }

    // Sort intervals by start time
    intervals.sort_by_key(|interval| interval.start);

    let mut result = Vec::new();
    result.push(intervals[0].clone());

    for interval in intervals.into_iter().skip(1) {
        let last = result.last_mut().unwrap();
        if interval.start <= last.end {
            // Overlapping intervals, merge them
            if interval.end > last.end {
                last.end = interval.end;
            }
        } else {
            // No overlap, add to result
            result.push(interval);
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read number of test cases
    let tc: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..tc {
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

        let merged = merge_overlap(intervals);

        for interval in merged {
            print!("{} {} ", interval.start, interval.end);
        }
        println!();
        println!("~");
    }
}
