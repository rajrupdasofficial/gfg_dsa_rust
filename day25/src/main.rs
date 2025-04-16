use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
struct Interval {
    start: i32,
    end: i32,
}

fn insert_interval(intervals: &[Interval], new_interval: Interval) -> Vec<Interval> {
    let mut result = Vec::with_capacity(intervals.len() + 1);
    let mut new_interval = new_interval;
    let mut inserted = false;

    for &interval in intervals {
        if interval.end < new_interval.start {
            // Current interval ends before new interval starts, add it to result
            result.push(interval);
        } else if interval.start <= new_interval.end {
            // Overlapping intervals, merge them
            new_interval.start = new_interval.start.min(interval.start);
            new_interval.end = new_interval.end.max(interval.end);
        } else {
            // Current interval starts after new interval ends
            if !inserted {
                result.push(new_interval);
                inserted = true;
            }
            result.push(interval);
        }
    }

    if !inserted {
        result.push(new_interval);
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
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

        let line = lines.next().unwrap().unwrap();
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let new_interval = Interval {
            start: parts[0],
            end: parts[1],
        };

        let ans = insert_interval(&intervals, new_interval);

        print!("[");
        for (i, interval) in ans.iter().enumerate() {
            print!("[{},{}]", interval.start, interval.end);
            if i != ans.len() - 1 {
                print!(",");
            }
        }
        println!("]");
        println!("~");
    }
}
