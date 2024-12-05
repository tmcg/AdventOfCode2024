
use advent::*;

struct SafetyManual {
    rules: Vec<SafetyManualRule>,
    updates: Vec<SafetyManualUpdate>,
}

struct SafetyManualRule {
    left: i64,
    right: i64,
}

struct SafetyManualUpdate {
    pages: Vec<i64>,
}

impl SafetyManual {
    fn breaks_rule(&self, a: i64, b: i64) -> bool {
        self.rules.iter().any(|x| x.left == b && x.right == a)
    }

    fn in_order(&self, rg: &[i64]) -> bool {
        if rg.len() > 1 {
            let a = rg[0];
            let b = rg[1];

            if self.breaks_rule(a, b) {
                return false;
            }
            return self.in_order(&rg[1..]);
        }
        true
    }
    
    fn fix_order(&self, rg: &mut Vec<i64>) {
        if rg.len() > 1 {
            for i in 0..rg.len() - 1 {
                let a = rg[i];
                let b = rg[i + 1];

                if self.breaks_rule(a, b) {
                    rg[i] = b;
                    rg[i + 1] = a;
                    self.fix_order(rg);
                    break;
                }
            }
        }
    }

    fn sum_part1(&self) -> i64 {
        self.updates.iter()
        .filter(|x| self.in_order(x.pages.as_slice()))
        .map(|x| &x.pages)
        //.inspect(|p| println!("{:?}", p))
        .map(|p| p[p.len() / 2])
        //.inspect(|p| println!("{:?}", p))
        .sum::<i64>()
    }

    fn sum_part2(&self) -> i64 {
        self.updates.iter()
        .filter(|x| !self.in_order(x.pages.as_slice()))
        .map(|x| &x.pages)
        //.inspect(|p| println!("{:?}", p))
        .map(|p| {
            let mut pages = p.clone();
            self.fix_order(&mut pages);
            pages
        })
        //.inspect(|p| println!("{:?}", p))
        .map(|p| p[p.len() / 2])
        .sum::<i64>()
    }
}

impl From<&str> for SafetyManual {
    fn from(s: &str) -> Self {
        let lines = input_as_lines(s);

        let rules = lines.iter()
            .filter(|x| x.contains("|"))
            .map(|x| SafetyManualRule::from(x.as_str()))
            .collect();

        let updates = lines.iter()
            .filter(|x| x.contains(","))
            .map(|x| SafetyManualUpdate::from(x.as_str()))
            .collect();

        SafetyManual { rules, updates }
    }
}

impl From<&str> for SafetyManualRule {
    fn from(s: &str) -> Self {
        let parts = s.split("|").collect::<Vec<_>>();
        if let [left, right] = parts[..] {
            let left = left.parse::<i64>().expect("Unable to parse left side");
            let right = right.parse::<i64>().expect("Unable to parse right side");
            return SafetyManualRule { left, right };
        }
        
        panic!("Unable to parse rule: {}", s);
    }
}

impl From<&str> for SafetyManualUpdate {
    fn from(s: &str) -> Self {
        let pages = s.split(",").map(|x| x.parse::<i64>().expect("Unable to parse page")).collect();
        SafetyManualUpdate { pages }
    }
}

fn default_input() -> &'static str {
    include_input!(05)
}

pub fn part1() -> String {
    let man = SafetyManual::from(default_input());
    man.sum_part1().to_string()
}

pub fn part2() -> String {
    let man = SafetyManual::from(default_input());
    man.sum_part2().to_string()
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
        let model = SafetyManual::from("1|2\r\n3|4\r\n5,6,7\r\n8,9,10");
        assert_eq!(model.rules.len(), 2);
        assert_eq!(model.updates.len(), 2);
        assert_eq!(model.rules[0].left, 1);
        assert_eq!(model.rules[0].right, 2);
        assert_eq!(model.rules[1].left, 3);
        assert_eq!(model.rules[1].right, 4);
        assert_eq!(model.updates[0].pages, vec![5, 6, 7]);
        assert_eq!(model.updates[1].pages, vec![8, 9, 10]);
    }

    #[test]
    fn in_order() {
        let rules = "47|53\r\n97|13\r\n97|61\r\n97|47\r\n75|29\r\n61|13\r\n75|53\r\n29|13\r\n97|29\r\n53|29\r\n61|53\r\n97|53\r\n61|29\r\n47|13\r\n75|47\r\n97|75\r\n47|61\r\n75|61\r\n47|29\r\n75|13\r\n53|13\r\n\r\n";

        let upd1 = "75,47,61,53,29";
        let upd2 = "97,61,53,29,13";
        let upd3 = "75,29,13";
        let upd4 = "75,97,47,61,53";
        let upd5 = "61,13,29";
        let upd6 = "97,13,75,29,47";

        let man1 = SafetyManual::from(format!("{}\r\n{}", rules, upd1).as_str());
        assert!(man1.in_order(man1.updates[0].pages.as_slice()));
        assert_eq!(man1.sum_part1(), 61);

        let man2 = SafetyManual::from(format!("{}\r\n{}", rules, upd2).as_str());
        assert!(man2.in_order(man2.updates[0].pages.as_slice()));
        assert_eq!(man2.sum_part1(), 53);

        let man3 = SafetyManual::from(format!("{}\r\n{}", rules, upd3).as_str());
        assert!(man3.in_order(man3.updates[0].pages.as_slice()));
        assert_eq!(man3.sum_part1(), 29);
        
        let man4 = SafetyManual::from(format!("{}\r\n{}", rules, upd4).as_str());
        assert!(!man4.in_order(man4.updates[0].pages.as_slice()));

        let man5 = SafetyManual::from(format!("{}\r\n{}", rules, upd5).as_str());
        assert!(!man5.in_order(man5.updates[0].pages.as_slice()));

        let man6 = SafetyManual::from(format!("{}\r\n{}", rules, upd6).as_str());
        assert!(!man6.in_order(man6.updates[0].pages.as_slice()));

        let man7 = SafetyManual::from(format!("{}\r\n{}\r\n{}\r\n{}\r\n{}\r\n{}\r\n{}", rules, upd1, upd2, upd3, upd4, upd5, upd6).as_str());
        assert_eq!(man7.sum_part1(), 61 + 53 + 29);
    }

    #[test]
    fn fix_order() {
        let rules = "47|53\r\n97|13\r\n97|61\r\n97|47\r\n75|29\r\n61|13\r\n75|53\r\n29|13\r\n97|29\r\n53|29\r\n61|53\r\n97|53\r\n61|29\r\n47|13\r\n75|47\r\n97|75\r\n47|61\r\n75|61\r\n47|29\r\n75|13\r\n53|13\r\n\r\n";

        let upd1 = "75,47,61,53,29";
        let upd2 = "97,61,53,29,13";
        let upd3 = "75,29,13";
        let upd4 = "75,97,47,61,53";
        let upd5 = "61,13,29";
        let upd6 = "97,13,75,29,47";

        let man4 = SafetyManual::from(format!("{}\r\n{}", rules, upd4).as_str());
        let mut pages4 = man4.updates[0].pages.clone();
        man4.fix_order(&mut pages4);
        assert_eq!(pages4, vec![97, 75, 47, 61, 53]);

        let man5 = SafetyManual::from(format!("{}\r\n{}", rules, upd5).as_str());
        let mut pages5 = man5.updates[0].pages.clone();
        man5.fix_order(&mut pages5);
        assert_eq!(pages5, vec![61, 29, 13]);

        let man6 = SafetyManual::from(format!("{}\r\n{}", rules, upd6).as_str());
        let mut pages6 = man6.updates[0].pages.clone();
        man6.fix_order(&mut pages6);
        assert_eq!(pages6, vec![97, 75, 47, 29, 13]);

        let man7 = SafetyManual::from(format!("{}\r\n{}\r\n{}\r\n{}\r\n{}\r\n{}\r\n{}", rules, upd1, upd2, upd3, upd4, upd5, upd6).as_str());
        assert_eq!(man7.sum_part2(), 47 + 29 + 47);
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "5275");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "6191");
    }
}
