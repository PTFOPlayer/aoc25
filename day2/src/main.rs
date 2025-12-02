fn main() {
    let data = include_str!("input.txt").split(",").map(|range| {
        let mut range = range.trim().split("-");
        let start = range.next().unwrap().parse::<i64>().unwrap();
        let end = range.next().unwrap().parse::<i64>().unwrap();
        (start, end)
    });

    let mut part1 = 0;
    let mut part2 = 0;

    data.for_each(|(start, end)| {
        for num in start..=end {
            let num_str = num.to_string();
            let len = num_str.len();
            // part 1
            if num_str[0..len / 2] == num_str[len / 2..] {
                part1 += num;
            }

            // part 2
            if is_repeatable(&num_str, len) {
                part2 += num;
            }
        }
    });

    println!("{}\n{}", part1, part2)
}

fn is_repeatable(num_str: &str, len: usize) -> bool {
    (1..=len / 2).filter(|mask| len % mask == 0).any(|mask| {
        let pattern = &num_str[0..mask];
        (mask..len)
            .step_by(mask)
            .all(|offset| &num_str[offset..offset + mask] == pattern)
    })
}
