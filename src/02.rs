
use advent::*;
use itertools::Itertools;
use std::cmp::Ordering;

struct InputModel {
    reports: Vec<Report>,
}

struct Report {
    levels: Vec<i64>,
}

impl From<&str> for InputModel {
    fn from(s: &str) -> Self {
        let reports = input_as_lines(s).iter()
            .map(|a| Report::from(a.as_str()))
            .collect::<Vec<_>>();
        InputModel { reports }
    }
}

impl From<&str> for Report {
    fn from(s: &str) -> Self {
        let levels = s.split(" ")
            .map(|b| b.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        Report { levels }
    }
}

impl Report {
    fn is_safe_part1(&self) -> bool {
        self.is_safe_impl(&self.levels)
    }

    fn is_safe_part2(&self) -> bool {
        self.levels.iter()
            .enumerate()
            .any(|(pos, _)| {
                let mut test = self.levels.clone();
                test.remove(pos);
                self.is_safe_impl(&test)
            })
    }

    fn is_safe_impl(&self, test: &[i64]) -> bool {
        let mut is_lt = false;
        let mut is_gt = false;

        test.iter()
            .tuple_windows()
            .for_each(|(a, b)| {
                match a.cmp(b) {
                    Ordering::Less => is_lt = true,
                    Ordering::Greater => is_gt = true,
                    _ => (),
                }
            });

        test.iter()
            .tuple_windows()
            .all(|(a, b)| {
                let x = (a - b).abs();
                (1..=3).contains(&x)
            })
        && (is_lt || is_gt)
        && !(is_lt && is_gt)
    }
}

fn default_input() -> &'static str {
    include_input!(02)
}

pub fn part1() -> String {
    let model = InputModel::from(default_input());

    model.reports.iter()
        .filter(|r| r.is_safe_part1())
        .count()
        .to_string()
}

pub fn part2() -> String {
    let model = InputModel::from(default_input());

    model.reports.iter()
        .filter(|r| r.is_safe_part2())
        .count()
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
    fn report_is_safe_part1() {
        let report = Report::from("7 6 4 2 1");
        assert!(report.is_safe_part1());
        let report = Report::from("1 3 6 7 9");
        assert!(report.is_safe_part1());
    }

    #[test]
    fn report_not_safe_part1() {
        let report = Report::from("1 2 7 8 9");
        assert!(!report.is_safe_part1());
        let report = Report::from("9 7 6 2 1");
        assert!(!report.is_safe_part1());
        let report = Report::from("1 3 2 4 5");
        assert!(!report.is_safe_part1());
        let report = Report::from("8 6 4 4 1");
        assert!(!report.is_safe_part1());
    }

    #[test]
    fn report_is_safe_part2() {
        let report = Report::from("7 6 4 2 1");
        assert!(report.is_safe_part2());
        let report = Report::from("1 3 2 4 5");
        assert!(report.is_safe_part2());
        let report = Report::from("8 6 4 4 1");
        assert!(report.is_safe_part2());
        let report = Report::from("1 3 6 7 9");
        assert!(report.is_safe_part2());
    }

    #[test]
    fn report_not_safe_part2() {
        let report = Report::from("1 2 7 8 9");
        assert!(!report.is_safe_part2());
        let report = Report::from("9 7 6 2 1");
        assert!(!report.is_safe_part2());
    }    

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "269");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "337");
    }
}
