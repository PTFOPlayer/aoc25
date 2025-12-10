use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let points = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.split(",")
                .map(|item| item.parse::<i64>().unwrap())
                .collect_tuple::<(i64, i64)>()
                .unwrap()
        })
        .collect_vec();

    part1(&points);
    part2(&points);
}

fn part1(points: &[(i64, i64)]) {
    let largest = points
        .iter()
        .array_combinations::<2>()
        .map(|[(x1, y1), (x2, y2)]| {
            let x = (x1 - x2 + 1).abs();
            let y = (y1 - y2 + 1).abs();

            x * y
        })
        .max()
        .unwrap();

    println!("{}", largest)
}

fn part2(points: &[(i64, i64)]) {
    let mut edges = HashSet::new();

    for (&(x1, y1), &(x2, y2)) in points.iter().circular_tuple_windows() {
        for p in x1.min(x2)..=x1.max(x2) {
            edges.insert((p, y1));
        }

        for p in y1.min(y2)..=y1.max(y2) {
            edges.insert((x1, p));
        }
    }

    let largest = points
        .iter()
        .array_combinations::<2>()
        .map(|[(x1, y1), (x2, y2)]| {
            let x = (x1 - x2).abs() + 1;
            let y = (y1 - y2).abs() + 1;

            let (&min_x, &max_x) = (x1.min(x2), x1.max(x2));
            let (&min_y, &max_y) = (y1.min(y2), y1.max(y2));

            let start_x = min_x + 1;
            let end_x = max_x - 1;

            let start_y = min_y + 1;
            let end_y = max_y - 1;

            for &(ex, ey) in &edges {
                if ex > start_x && ex < end_x && ey > start_y && ey < end_y {
                    return 0;
                }
            }

            x * y
        })
        .max()
        .unwrap();

    println!("{}", largest)
}
