
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

impl BridgeCalibration {
    const OP_ADD: u8 = 0b001;
    const OP_MUL: u8 = 0b010;
    const OP_CON: u8 = 0b100;

    fn op_concat(a: i64, b: i64) -> i64 {
        let bl = b.checked_ilog10().unwrap_or(0);
        let bp = i64::pow(10, bl + 1);
        a * bp + b
    }
}

impl BridgeEquation {
    fn calc_part1(&self) -> i64 {
        self.calc(BridgeCalibration::OP_ADD | BridgeCalibration::OP_MUL)
    }

    fn calc_part2(&self) -> i64 {
        self.calc(BridgeCalibration::OP_ADD | BridgeCalibration::OP_MUL | BridgeCalibration::OP_CON)
    }

    fn calc(&self, ops: u8) -> i64 {
        let v0 = self.values[0];
        let v1 = &self.values[1..];
        self.calc_impl(v0, v1, &mut 0, ops)
    }

    fn calc_impl(&self, acc: i64, values: &[i64], cref: &mut i64, ops: u8) -> i64 {
        let op_add = ops & BridgeCalibration::OP_ADD > 0;
        let op_mul = ops & BridgeCalibration::OP_MUL > 0;
        let op_con = ops & BridgeCalibration::OP_CON > 0;
        let v0 = values[0];
        let v1 = &values[1..];
        if v1.is_empty() {
            if op_add && acc + v0 == self.result { *cref +=1 };
            if op_mul && acc * v0 == self.result { *cref +=1 };
            if op_con && BridgeCalibration::op_concat(acc, v0) == self.result { *cref +=1 };
        } else {
            if op_add { self.calc_impl(acc + v0, v1, cref, ops); }
            if op_mul { self.calc_impl(acc * v0, v1, cref, ops); }
            if op_con { self.calc_impl(BridgeCalibration::op_concat(acc, v0), v1, cref, ops); }
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
    let model = BridgeCalibration::from(default_input());

    model.equations.iter()
        .filter(|x| x.calc_part2() > 0)
        .map(|x| x.result)
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
    fn eval_part2() {
        let c = BridgeCalibration::from("190: 10 19\r\n3267: 81 40 27\r\n83: 17 5\r\n156: 15 6\r\n7290: 6 8 6 15\r\n161011: 16 10 13\r\n192: 17 8 14\r\n21037: 9 7 18 13\r\n292: 11 6 16 20");

        assert_eq!(c.equations.len(), 9);
        assert_eq!(c.equations[0].calc_part2(), 1);
        assert_eq!(c.equations[1].calc_part2(), 2);
        assert_eq!(c.equations[2].calc_part2(), 0);
        assert_eq!(c.equations[3].calc_part2(), 1);
        assert_eq!(c.equations[4].calc_part2(), 1);
        assert_eq!(c.equations[5].calc_part2(), 0);
        assert_eq!(c.equations[6].calc_part2(), 1);
        assert_eq!(c.equations[7].calc_part2(), 0);
        assert_eq!(c.equations[8].calc_part2(), 1);

        let total = c.equations.iter()
            .filter(|x| x.calc_part2() > 0)
            .map(|x| x.result)
            .sum::<i64>();

        assert_eq!(total, 190 + 3267 + 292 + 156 + 7290 + 192);
    }

    #[test]
    fn op_concat() {
        assert_eq!(BridgeCalibration::op_concat(1, 2), 12);
        assert_eq!(BridgeCalibration::op_concat(15, 2), 152);
        assert_eq!(BridgeCalibration::op_concat(15, 22), 1522);
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "1620690235709");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "145397611075341");
    }
}
