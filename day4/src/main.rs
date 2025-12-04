use itertools::Itertools;

fn main() {
    let grid = include_str!("input.txt")
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect_vec();

    let kernel = [
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, 1),
        (0, -1),
    ];

    part1(&grid, &kernel);
    part2(grid, kernel);
}

fn part1(grid: &Vec<Vec<u8>>, kernel: &[(isize, isize); 8]) {
    let mut positions = 0;

    for height in 0..grid.len() {
        for width in 0..grid[height].len() {
            if grid[height][width] != b'@' {
                continue;
            }

            let height = height as isize;
            let width = width as isize;

            let free = check_position(grid, kernel, height, width);

            if free < 4 {
                positions += 1;
            }
        }
    }

    println!("{}", positions);
}

fn part2(mut grid: Vec<Vec<u8>>, kernel: [(isize, isize); 8]) {
    let mut positions = 0;
    let mut removed_pass = 1;
    while removed_pass != 0 {
        removed_pass = 0;
        for height in 0..grid.len() {
            for width in 0..grid[height].len() {
                if grid[height][width] != b'@' {
                    continue;
                }

                let iheight = height as isize;
                let iwidth = width as isize;

                let free = check_position(&grid, &kernel, iheight, iwidth);

                if free < 4 {
                    removed_pass += 1;
                    grid[height][width] = b'x';
                }
            }
        }

        positions += removed_pass;
    }

    println!("{}", positions);
}

fn check_position(
    grid: &Vec<Vec<u8>>,
    kernel: &[(isize, isize); 8],
    height: isize,
    width: isize,
) -> i32 {
    kernel
        .iter()
        .map(|(x, y)| {
            let target_y = height + y;
            let Ok(target_y) = TryInto::<usize>::try_into(target_y) else {
                return 0i32;
            };

            let Some(row) = grid.get(target_y) else {
                return 0i32;
            };

            let target_x = width + x;
            let Ok(target_x) = TryInto::<usize>::try_into(target_x) else {
                return 0i32;
            };

            let Some(item) = row.get(target_x) else {
                return 0i32;
            };

            if *item != b'@' {
                return 0;
            }

            return 1;
        })
        .sum::<i32>()
}
