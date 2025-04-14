use std::io;

fn merge_sort(arr: &mut [i32], temp: &mut [i32], left: usize, right: usize) -> i64 {
    let mut inversions = 0;
    if left < right {
        let mid = left + (right - left) / 2;
        inversions += merge_sort(arr, temp, left, mid);
        inversions += merge_sort(arr, temp, mid + 1, right);

        let mut i = left;
        let mut j = mid + 1;
        let mut k = left;
        while i <= mid && j <= right {
            if arr[i] <= arr[j] {
                temp[k] = arr[i];
                i += 1;
            } else {
                temp[k] = arr[j];
                j += 1;
                inversions += (mid - i + 1) as i64;
            }
            k += 1;
        }
        while i <= mid {
            temp[k] = arr[i];
            i += 1;
            k += 1;
        }
        while j <= right {
            temp[k] = arr[j];
            j += 1;
            k += 1;
        }
        for i in left..=right {
            arr[i] = temp[i];
        }
    }
    inversions
}

fn count_inversions(arr: &mut [i32]) -> i64 {
    let mut temp = vec![0; arr.len()];
    merge_sort(arr, &mut temp, 0, arr.len() - 1)
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let nums: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input"))
            .collect();

        let mut arr = nums.clone();
        let ans = count_inversions(&mut arr);
        println!("{}", ans);
    }
}
