
use advent::*;

fn default_input() -> &'static str {
    include_input!(01)
}

fn line_or_empty(lines: &[String], index: usize) -> String {
    if lines.len() > index {
        return lines[index].to_string()
    }
    String::from("")
}

pub fn part1() -> String {
    let lines = input_as_lines(default_input());
    line_or_empty(&lines, 0)
}

pub fn part2() -> String {
    let lines = input_as_lines(default_input());
    line_or_empty(&lines, 1)
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
        assert_eq!(part1(), "aa1");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "aa2");
    }
}
