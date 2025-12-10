use glam::Vec3;
use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use std::collections::HashMap;

fn main() {
    let boxes = include_str!("input.txt")
        .lines()
        .map(|line| {
            Vec3::from_array(
                line.split(",")
                    .map(|item| item.parse::<f32>().unwrap())
                    .collect_array::<3>()
                    .unwrap(),
            )
        })
        .collect_vec();

    let n = boxes.len();

    let mut pairs = Vec::new();

    for i in 0..n {
        for j in (i + 1)..n {
            pairs.push((i, j, boxes[i].distance(boxes[j])));
        }
    }

    pairs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    part1(n, &pairs);
    part2(&boxes, &pairs);
}

fn part1(n: usize, pairs: &[(usize, usize, f32)]) {
    // 10 for example, 1000 for actual input
    let max_connections = if n == 20 { 10 } else { 1000 };

    let pairs = &pairs[..max_connections];

    let mut uf = UnionFind::new(n);

    for &(i, j, _) in pairs {
        uf.union(i, j);
    }

    let mut groups = HashMap::new();

    for i in 0..n {
        let r = uf.find(i);
        *groups.entry(r).or_insert(0) += 1;
    }

    let result: i32 = groups
        .iter()
        .map(|(_, boxes)| boxes)
        .sorted()
        .rev()
        .take(3)
        .product();

    println!("{}", result)
}

fn part2(boxes: &[Vec3], pairs: &[(usize, usize, f32)]) {
    let n = boxes.len();
    let mut uf = UnionFind::new(n);

    let mut last = (0, 0);

    pairs.into_iter().for_each(|&(i, j, _)| {
        let a = uf.find(i);
        let b = uf.find(j);

        if a != b {
            uf.union(a, b);
            last = (i, j);
        }
    });

    let (i, j) = last;

    let result = boxes[i].x as i64 * boxes[j].x as i64;

    println!("{}", result);
}
