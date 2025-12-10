use std::ops::Deref;

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Button(pub Vec<usize>);

impl Deref for Button {
    type Target = [usize];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct Machine {
    pub target: Vec<bool>,
    pub buttons: Vec<Button>,
    pub joltage: Vec<usize>,
}

impl Machine {
    pub fn from_str(line: &str) -> Self {
        let mut machine = Machine {
            target: vec![],
            buttons: vec![],
            joltage: vec![],
        };

        for item in line.split_whitespace() {
            match item.as_bytes().first() {
                Some(b'[') => machine.target_from_str(item),
                Some(b'(') => machine.add_button_from_str(item),
                Some(b'{') => machine.joltage_from_str(item),
                sym => panic!("unknown symbol: {:?}", sym),
            }
        }

        machine
    }

    fn target_from_str(&mut self, str: &str) {
        assert!(str.starts_with("[") && str.ends_with("]"));

        let data = str[1..str.len() - 1]
            .as_bytes()
            .iter()
            .map(|byte| match byte {
                b'#' => true,
                b'.' => false,
                sym => panic!("unknown symbol in target: {:?}", sym),
            })
            .collect_vec();

        self.target = data;
    }

    fn add_button_from_str(&mut self, str: &str) {
        assert!(str.starts_with("(") && str.ends_with(")"));

        let data = str[1..str.len() - 1]
            .split(",")
            .map(|item| item.parse::<usize>().unwrap())
            .collect_vec();

        self.buttons.push(Button(data));
    }

    fn joltage_from_str(&mut self, str: &str) {
        assert!(str.starts_with("{") && str.ends_with("}"));

        let data = str[1..str.len() - 1]
            .split(",")
            .map(|item| item.parse::<usize>().unwrap())
            .collect_vec();

        self.joltage = data;
    }
}
