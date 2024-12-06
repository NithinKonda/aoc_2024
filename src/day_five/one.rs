use std::fs;

#[derive(Debug)]
struct Rule {
    first: i32,
    second: i32,
}

impl Rule {
    fn new(data: &str) -> Self {
        let numbers: Vec<i32> = data.split('|').filter_map(|x| x.parse().ok()).collect();
        Rule {
            first: numbers[0],
            second: numbers[1],
        }
    }
}

#[derive(Debug)]
struct Update {
    numbers: Vec<i32>,
}

impl Update {
    fn new(data: &str) -> Self {
        Update {
            numbers: data.split(',').filter_map(|x| x.parse().ok()).collect(),
        }
    }

    fn is_valid_against_rule(&self, rule: &Rule) -> bool {
        let first_index = self.numbers.iter().position(|&x| x == rule.first);
        let second_index = self.numbers.iter().position(|&x| x == rule.second);

        match (first_index, second_index) {
            (Some(first), Some(second)) => first < second,
            _ => true, // Rule does not apply if either number is missing
        }
    }

    fn is_valid_against_rules(&self, rules: &[Rule]) -> bool {
        rules.iter().all(|rule| self.is_valid_against_rule(rule))
    }

    fn get_middle_number(&self) -> Option<i32> {
        let len = self.numbers.len();
        if len % 2 == 1 {
            Some(self.numbers[len / 2])
        } else {
            None
        }
    }
}

fn main() {
    let input = fs::read_to_string("data.txt").expect("Failed to read input file");
    let mut rules = Vec::new();
    let mut updates = Vec::new();
    let mut parsing_rules = true;

    for line in input.lines() {
        if line.trim().is_empty() {
            parsing_rules = false;
        } else if parsing_rules {
            rules.push(Rule::new(line));
        } else {
            updates.push(Update::new(line));
        }
    }

    let mut result = 0;
    for update in &updates {
        if update.is_valid_against_rules(&rules) {
            if let Some(middle) = update.get_middle_number() {
                result += middle;
            }
        }
    }

    println!("Part 1 Result: {}", result);
}
