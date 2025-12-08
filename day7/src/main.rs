use ahash::AHashMap;
use itertools::Itertools;

type Positions = AHashMap<(usize, usize), i64>;

fn main() {
    let grid = include_str!("input.txt")
        .lines()
        .map(|line| line.as_bytes())
        .collect_vec();

    let s_position = grid[0].iter().position(|b| *b == b'S').unwrap();

    let mut positions = Positions::with_capacity(2048);

    let paths = recursive(s_position, 0, &grid, &mut positions);

    println!("{}", positions.len());
    println!("{}", paths)
}

fn recursive(x: usize, y: usize, grid: &[&[u8]], positions: &mut Positions) -> i64 {
    if y + 1 >= grid.len() {
        return 1;
    }

    if let Some(val) = positions.get(&(x, y + 1)) {
        return *val;
    }

    return match grid[y + 1][x] {
        b'.' => recursive(x, y + 1, grid, positions),
        b'^' => {
            let sum = recursive(x - 1, y, grid, positions) + recursive(x + 1, y, grid, positions);

            positions.insert((x, y + 1), sum);
            sum
        }
        _ => panic!("Unknown symbol"),
    };
}
