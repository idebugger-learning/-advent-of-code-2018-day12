use std::collections::HashMap;

use regex::Regex;

fn main() {
    // Example
    // let initial_state = "#..#.#..##......###...###";
    // let rules = include_str!("./data/input_example.txt");

    // Input
    let initial_state = "##.##.##..#..#.#.#.#...#...#####.###...#####.##..#####.#..#.##..#..#.#...#...##.##...#.##......####.";
    let rules = include_str!("./data/input.txt");

    let rules = parse_rules(rules);

    let mut state = parse_initial_state(initial_state);
    for _ in 0..20 {
        state = run_step(state, &rules);
    }

    let sum_of_numbers = state
        .iter()
        .filter(|(_, &value)| value)
        .map(|(&key, _)| key)
        .sum::<isize>();
    println!("Sum of numbers: {}", sum_of_numbers);
}

fn run_step(state: HashMap<isize, bool>, rules: &HashMap<[bool; 5], bool>) -> HashMap<isize, bool> {
    let lower_plant = *state.keys().min().unwrap();
    let higher_plant = *state.keys().max().unwrap();

    let from = lower_plant - 3;
    let to = higher_plant + 3;

    let mut new_state = HashMap::new();
    for i in from..=to {
        let snapshot = [
            *state.get(&(i - 2)).unwrap_or(&false),
            *state.get(&(i - 1)).unwrap_or(&false),
            *state.get(&i).unwrap_or(&false),
            *state.get(&(i + 1)).unwrap_or(&false),
            *state.get(&(i + 2)).unwrap_or(&false),
        ];
        let target = *rules.get(&snapshot).unwrap_or(&false);
        new_state.insert(i, target);
    }
    new_state
}

fn parse_initial_state(input: &str) -> HashMap<isize, bool> {
    let mut map = HashMap::new();
    for (i, chr) in input.chars().enumerate() {
        map.insert(i as isize, chr == '#');
    }
    map
}

fn parse_rules(input: &str) -> HashMap<[bool; 5], bool> {
    let rule_parser = Regex::new(r"([.#]{5}) => ([.#])").unwrap();

    let mut rules = HashMap::new();
    for rule in input.split("\n") {
        let rule_captures = rule_parser.captures(rule).unwrap();
        let rule_pattern = rule_captures.get(1).unwrap().as_str();
        let rule_target = rule_captures.get(2).unwrap().as_str();

        let mut rule_pattern_chars = rule_pattern.chars().map(|chr| chr == '#');
        let rule_pattern = [
            rule_pattern_chars.next().unwrap(),
            rule_pattern_chars.next().unwrap(),
            rule_pattern_chars.next().unwrap(),
            rule_pattern_chars.next().unwrap(),
            rule_pattern_chars.next().unwrap(),
        ];
        let rule_target = rule_target.chars().next().unwrap() == '#';

        rules.insert(rule_pattern, rule_target);
    }
    rules
}
