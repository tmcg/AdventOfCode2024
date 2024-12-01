
use advent::*;
use itertools::Itertools;

struct InputModel {
    lines: Vec<(i64, i64)>,
}

impl InputModel {
    fn left(&self) -> Vec<i64> {
        self.lines.iter().map(|x| x.0).collect::<Vec<_>>()
    }
    fn right(&self) -> Vec<i64> {
        self.lines.iter().map(|x| x.1).collect::<Vec<_>>()
    }
}

impl From<&str> for InputModel {

    fn from(s: &str) -> Self {
        let lines = input_as_lines(s).iter()
            .map(|a| a.split("   ").map(|b| b.parse::<i64>().unwrap()).collect::<Vec<_>>())
            .map(|b| (b[0], b[1]))
            .collect::<Vec<_>>();

        InputModel { lines }
    }
}

fn default_input() -> &'static str {
    include_input!(01)
}

pub fn part1() -> String {
    let model = InputModel::from(default_input());

    let left = model.left().iter().sorted().cloned().collect::<Vec<_>>();
    let right = model.right().iter().sorted().cloned().collect::<Vec<_>>();

    let sum = left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i64>();

    sum.to_string()
}

pub fn part2() -> String {
    let model = InputModel::from(default_input());

    let left = model.left();
    let right = model.right();

    let sum = left.iter()
        .map(|a| { *a * right.iter().filter(|b| **b == *a).count() as i64 })
        .sum::<i64>();

    sum.to_string()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "2176849");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "23384288");
    }
}
