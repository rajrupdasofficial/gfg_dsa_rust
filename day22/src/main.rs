fn h_index(citations: Vec<i32>) -> i32 {
    let n = citations.len() as i32;
    let mut freq = vec![0; (n + 1) as usize];

    for &citation in &citations {
        if citation >= n {
            freq[n as usize] += 1;
        } else {
            freq[citation as usize] += 1;
        }
    }

    let mut h_index = n;
    let mut num = freq[n as usize];

    while num < h_index {
        h_index -= 1;
        num += freq[h_index as usize];
    }

    h_index
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let citations: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let ans = h_index(citations);
        println!("{}", ans);
    }
}
