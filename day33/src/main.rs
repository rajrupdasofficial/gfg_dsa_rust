impl Solution {
    pub fn aggressive_cows(mut stalls: Vec<i32>, k: i32) -> i32 {
        stalls.sort();
        let mut low = 1;
        let mut high = stalls[stalls.len() - 1] - stalls[0];
        let mut ans = 0;

        while low <= high {
            let mid = low + (high - low) / 2;
            let mut count = 1;
            let mut last = stalls[0];

            for &stall in stalls.iter().skip(1) {
                if stall - last >= mid {
                    count += 1;
                    last = stall;
                }
            }

            if count >= k {
                ans = mid;
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }

        ans
    }
}
fn main() {
    let stalls = vec![1, 2, 8, 4, 9];
    let k = 3;
    let result = Solution::aggressive_cows(stalls, k);
    println!("{}", result); // Output should be 3
}
