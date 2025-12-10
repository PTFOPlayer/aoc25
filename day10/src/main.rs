mod machine;

use itertools::Itertools;
use pathfinding::directed::bfs;

use crate::machine::Machine;

fn main() {
    let machines = include_str!("input.txt")
        .lines()
        .map(|line| Machine::from_str(line))
        .collect_vec();

    part1(&machines);
}

fn part1(machines: &[Machine]) {
    let total = machines
        .iter()
        .map(|machine| {
            let start = vec![false; machine.target.len()];
            let path = bfs::bfs(
                &start,
                |a| {
                    machine
                        .buttons
                        .iter()
                        .map(|button| {
                            let mut next = a.clone();

                            for &change in button.iter() {
                                next[change] = !next[change];
                            }

                            next
                        })
                        .collect_vec()
                },
                |arg| arg == &machine.target,
            )
            .expect("Path should always be possible");

            path.len() - 1
        })
        .sum::<usize>();

    println!("{}", total)
}
