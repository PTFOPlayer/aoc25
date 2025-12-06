use itertools::Itertools;

fn main() {
    let mut input = include_str!("input.txt").lines().rev();

    let operators = input
        .next()
        .expect("Operators line not found")
        .split_whitespace()
        .filter(|s| !s.trim().is_empty())
        .collect_vec();

    let data = input.rev().collect_vec();

    part1(data.clone(), operators.clone());
    part2(data, operators);
}

fn part1(data: Vec<&str>, operators: Vec<&str>) {
    let mut input = data.iter();

    let mut accumulators = input
        .next()
        .expect("Accumulators line not found")
        .split_whitespace()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.parse::<i64>().unwrap())
        .collect_vec();

    input.for_each(|line| {
        let numbers = line
            .split_whitespace()
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.parse::<i64>().unwrap());

        accumulators
            .iter_mut()
            .zip(operators.iter())
            .zip(numbers)
            .for_each(|((accumulator, operator), number)| match *operator {
                "+" => *accumulator += number,
                "*" => *accumulator *= number,
                _ => panic!("Unknown operator: {operator}"),
            });
    });

    let sum = accumulators.iter().sum::<i64>();

    println!("{}", sum);
}

fn part2(data: Vec<&str>, operators: Vec<&str>) {
    let data = data.into_iter().map(|line| line.as_bytes()).collect_vec();

    let mut reconstructed = vec![vec![]];

    let mut offset = 0;

    while data.iter().any(|line| line.len() > offset) {
        let mut constructed = 0i64;
        for line in data.iter() {
            let Some(item) = line.get(offset) else {
                continue;
            };

            let number = match item {
                b'0'..=b'9' => (item - b'0') as i64,
                _ => continue,
            };

            constructed = constructed * 10 + number;
        }
        offset += 1;

        if constructed == 0 {
            reconstructed.push(vec![]);
        } else {
            reconstructed.last_mut().unwrap().push(constructed);
        }
    }

    let result = reconstructed
        .into_iter()
        .zip(operators)
        .map(|(column, operator)| {
            let mut column_iter = column.into_iter();
            let acc = column_iter.next().unwrap();
            column_iter.fold(acc, |acc, item| match operator {
                "+" => acc + item,
                "*" => acc * item,
                _ => panic!("invalid operator"),
            })
        })
        .sum::<i64>();

    println!("{}", result)
}
