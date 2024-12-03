
use advent::*;
use regex::Regex;

struct Memory {
    line: String,
}

struct Multiply {
    x: i64,
    y: i64,
}

impl From<&str> for Memory {
    fn from(s: &str) -> Self {
        Memory { line: s.replace("\n","").to_owned() }
    }
}

impl Memory {
    fn parse_part1(&self) -> Vec<Multiply> {
        self.parse_impl(&self.formatted_part1())
    }
    
    fn formatted_part1(&self) -> String {
        self.line.clone()
    }

    fn parse_part2(&self) -> Vec<Multiply> {
        self.parse_impl(&self.formatted_part2())
    }

    fn formatted_part2(&self) -> String {
        let s = self.line.clone() + "do()";
        self.regex_on_off()
            .replace_all(&s, "")
            .to_string()
    }

    fn parse_impl(&self, s: &str) -> Vec<Multiply> {
        self.regex_multiply()
            .captures_iter(s)
            .map(|x| Multiply {
                x: x[1].parse::<i64>().unwrap(),
                y: x[2].parse::<i64>().unwrap(),
            })
            .collect::<Vec<_>>()
    }

    fn regex_multiply(&self) -> Regex {
        Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex for multiply")
    }
    fn regex_on_off(&self) -> Regex {
        Regex::new(r"don't\(\).*?do\(\)").expect("Invalid regex for on/off")
    }
}

fn default_input() -> &'static str {
    include_input!(03)
}

pub fn part1() -> String {
    let mem = Memory::from(default_input());
    mem.parse_part1().iter()
        .map(|b| b.x * b.y)
        .sum::<i64>()
        .to_string()
}

pub fn part2() -> String {
    let mem = Memory::from(default_input());
    mem.parse_part2().iter()
        .map(|b| b.x * b.y)
        .sum::<i64>()
        .to_string()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mem = Memory::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");

        let p = mem.parse_part1();
        assert_eq!(p.len(), 4);
        assert_eq!(p[0].x, 2);
        assert_eq!(p[0].y, 4);
        assert_eq!(p[1].x, 5);
        assert_eq!(p[1].y, 5);
        assert_eq!(p[2].x, 11);
        assert_eq!(p[2].y, 8);
        assert_eq!(p[3].x, 8);
        assert_eq!(p[3].y, 5);
    }

    #[test]
    fn test_part2() {
        let mem = Memory::from("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");

        let p = mem.parse_part2();
        assert_eq!(p.len(), 2);
        assert_eq!(p[0].x, 2);
        assert_eq!(p[0].y, 4);
        assert_eq!(p[1].x, 8);
        assert_eq!(p[1].y, 5);
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "166357705");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "88811886");
    }
}
