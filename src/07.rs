
use advent::*;

struct BridgeCalibration {
    equations: Vec<BridgeEquation>,
}

struct BridgeEquation {
    result: i64,
    values: Vec<i64>,
}

impl From<&str> for BridgeCalibration {
    fn from(s: &str) -> Self {
        let equations = input_as_lines(s).iter()
            .map(|a| BridgeEquation::from(a.as_str()))
            .collect::<Vec<_>>();
        BridgeCalibration { equations }
    }
}

impl From<&str> for BridgeEquation {
    fn from(s: &str) -> Self {
        let s = s.split(": ").collect::<Vec<_>>();
        let result = s[0].parse::<i64>().unwrap();
        let values = s[1].split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
        BridgeEquation { result, values }
    }
}

impl BridgeEquation {
    fn calc_part1(&self) -> i64 {
        let v0 = self.values[0];
        let v1 = &self.values[1..];
        self.calc_part1_impl(v0, v1, &mut 0)
    }

    fn calc_part1_impl(&self, acc: i64, values: &[i64], cref: &mut i64) -> i64 {
        if values.len() == 1 {
            if acc * values[0] == self.result { *cref += 1; }
            if acc + values[0] == self.result { *cref += 1; } 
        } else {
            let v0 = values[0];
            let v1 = &values[1..];
            self.calc_part1_impl(acc * v0, v1, cref);
            self.calc_part1_impl(acc + v0, v1, cref);
        }
        *cref
    }
}

fn default_input() -> &'static str {
    include_input!(07)
}

pub fn part1() -> String {
    let model = BridgeCalibration::from(default_input());

    model.equations.iter()
        .filter(|x| x.calc_part1() > 0)
        .map(|x| x.result)
        .sum::<i64>()
        .to_string()
}

pub fn part2() -> String {
    //let model = InputModel::from(default_input());
    String::from("zz")
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let m = BridgeEquation::from("190: 10 19");
        assert_eq!(m.result, 190);
        assert_eq!(m.values, vec![10, 19]);

        let m = BridgeEquation::from("3267: 81 40 27");
        assert_eq!(m.result, 3267);
        assert_eq!(m.values, vec![81, 40, 27]);
    }

    #[test]
    fn eval_part1() {
        let c = BridgeCalibration::from("190: 10 19\r\n3267: 81 40 27\r\n83: 17 5\r\n156: 15 6\r\n7290: 6 8 6 15\r\n161011: 16 10 13\r\n192: 17 8 14\r\n21037: 9 7 18 13\r\n292: 11 6 16 20");

        assert_eq!(c.equations.len(), 9);
        assert_eq!(c.equations[0].calc_part1(), 1);
        assert_eq!(c.equations[1].calc_part1(), 2);
        assert_eq!(c.equations[2].calc_part1(), 0);
        assert_eq!(c.equations[3].calc_part1(), 0);
        assert_eq!(c.equations[4].calc_part1(), 0);
        assert_eq!(c.equations[5].calc_part1(), 0);
        assert_eq!(c.equations[6].calc_part1(), 0);
        assert_eq!(c.equations[7].calc_part1(), 0);
        assert_eq!(c.equations[8].calc_part1(), 1);

        let total = c.equations.iter()
            .filter(|x| x.calc_part1() > 0)
            .map(|x| x.result)
            .sum::<i64>();

        assert_eq!(total, 190 + 3267 + 292);
    }


    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "1620690235709");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "zz");
    }
}
