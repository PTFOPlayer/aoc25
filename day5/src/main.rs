use itertools::Itertools;

fn main() {
    let mut ranges = vec![];

    let mut available = vec![];

    include_str!("input.txt").lines().for_each(|line| {
        let line = line.trim();
        if line.is_empty() {
            return;
        }

        if line.contains("-") {
            let (start, end) = line
                .split("-")
                .map(|item| item.parse::<i64>().expect("Parsing error in range"))
                .collect_tuple::<(i64, i64)>()
                .unwrap();

            ranges.push(start..=end);
        } else {
            available.push(line.parse::<i64>().expect("Parsing error in available"));
        }
    });

    let fresh_available = available
        .iter()
        .filter(|item| ranges.iter().any(|range| range.contains(item)))
        .count();

    let (mut total, current_start, current_end) = ranges
        .into_iter()
        .map(|range| (*range.start(), *range.end()))
        .sorted_by(|(a_start, _), (b_start, _)| Ord::cmp(a_start, b_start))
        .fold(
            (0, 0, 0),
            |(total, current_start, current_end), (start, end)| {
                if start <= current_end + 1 {
                    (total, current_start, current_end.max(end))
                } else {
                    (total + current_end - current_start + 1, start, end)
                }
            },
        );

    total += current_end - current_start;

    println!("{}", fresh_available);
    println!("{}", total)
}
