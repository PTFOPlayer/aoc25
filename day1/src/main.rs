fn main() {
    let (_, threw_zero_passes, zeros) = include_str!("input.txt")
        .lines()
        .map(|line| match line.chars().nth(0).unwrap() {
            'R' => line[1..].parse::<i32>().unwrap(),
            'L' => -line[1..].parse::<i32>().unwrap(),
            _ => panic!("Invalid direction"),
        })
        .fold(
            (50, 0, 0),
            |(position, mut threw_zero_passes, mut zeros), entry| {
                let new_position = position + entry;

                threw_zero_passes += (new_position / 100).abs();

                if position != 0 && new_position <= 0 {
                    threw_zero_passes += 1;
                }

                let position = new_position.rem_euclid(100);

                if position == 0 {
                    zeros += 1;
                }

                (position, threw_zero_passes, zeros)
            },
        );

    println!("Part1: {}", zeros);
    println!("Part2: {}", threw_zero_passes);
}