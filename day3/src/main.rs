use itertools::Itertools;
fn main() {
    let (part1, part2) = include_str!("input.txt")
        .lines()
        .fold((0, 0), |(part1, part2), line| {
            let nums = line
                .as_bytes()
                .iter()
                .map(|b| (b - b'0') as i64)
                .collect_vec();

            let largest_pair = max_subsequence(&nums, 2)
                .iter()
                .fold(0_i64, |acc, &d| acc * 10 + d);

            let max_12 = max_subsequence(&nums, 12)
                .iter()
                .fold(0_i64, |acc, &d| acc * 10 + d);

            (part1 + largest_pair, part2 + max_12)
        });

    println!("{}", part1);
    println!("{}", part2);
}

fn max_subsequence(nums: &[i64], k: usize) -> Vec<i64> {
    let mut stack = Vec::with_capacity(k);
    let mut to_remove = nums.len() - k;

    for &d in nums {
        while to_remove > 0 && !stack.is_empty() && stack[stack.len() - 1] < d {
            stack.pop();
            to_remove -= 1;
        }
        stack.push(d);
    }

    stack.truncate(k);
    stack
}
