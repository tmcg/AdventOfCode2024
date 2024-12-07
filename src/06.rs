
use advent::*;
use std::collections::{HashSet, HashMap};

#[derive(Debug, Default, Clone)]
struct NorthPoleMap {
    chars: HashMap<Point32, char>,
    width: i32,
    height: i32,
}

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
struct NorthPoleGuard {
    pos: Point32,
    dir: Compass,
}

#[derive(Debug, Default)]
struct NorthPoleLab {
    map: NorthPoleMap,
    guard: NorthPoleGuard
}

#[derive(Debug)]
enum NorthPoleErr {
    CycleDetected,
}

impl NorthPoleMap {
    fn obstruct_at(&self, p: Point32) -> Self {
        let mut result = self.clone();
        result.chars.insert(p, '#');
        result
    }
}

impl NorthPoleGuard {
    fn next_pos(&self) -> Point32 {
        let mut result = self.pos;
        match self.dir {
            Compass::North => result.y -= 1,
            Compass::East => result.x += 1,
            Compass::South => result.y += 1,
            Compass::West => result.x -= 1,
            _ => (),
        };
        result
    }
}

impl NorthPoleLab {
    fn walk_key(&self, pos: &Point32, dir: &Compass, part: u32) -> i32 {
        match part {
            1 => pos.y | pos.x << 10,
            _ => pos.y | pos.x << 10 | (*dir as i32) << 20,
        }
    }

    fn walk_part1(&self) -> usize {
        self.walk_impl(&self.map, 1).expect("Part 1 should not fail").len()
    }

    fn walk_part2(&self) -> usize {
        let pt1 = self.walk_impl(&self.map, 1).expect("Part 1 should not fail");

        let mut count: usize = 0;
        for y in 0..self.map.height - 1 {
            //println!("Row={}", y);
            for x in 0..self.map.width - 1 {
                let p = Point32 { x, y };

                // Optimisation: only consider points from part 1
                let key = self.walk_key(&p, &Compass::North, 1);
                if !pt1.contains(&key) {
                    continue;
                }

                let c = self.map.chars[&p];
                if c != '#' && c != '^' {
                    let map = self.map.obstruct_at(p);
                    if self.walk_impl(&map, 2).is_err() {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn walk_impl(&self, map: &NorthPoleMap, part: u32) -> Result<HashSet<i32>, NorthPoleErr> {
        let mut gcurr = self.guard;
        let mut walk = HashSet::new();
        walk.insert(self.walk_key(&gcurr.pos, &gcurr.dir, part));

        loop {
            let mut gnext = gcurr;
            gnext.pos = gnext.next_pos();

            if map.chars.contains_key(&gnext.pos) {
                //println!("{:?}: ch={} (dir={:?})", next, ch[&next], dir);
                if map.chars[&gnext.pos] == '#' {
                    gcurr.dir = gcurr.dir.cardinal_right();
                } else {
                    let key = self.walk_key(&gnext.pos, &gnext.dir, part);
                    if part == 2 && walk.contains(&key) {
                        return Err(NorthPoleErr::CycleDetected);
                    }

                    walk.insert(key);
                    gcurr.pos = gnext.pos;
                }

                continue;
            }

            break;
        }

        Ok(walk)
    }
}

impl From<&str> for NorthPoleLab {
    fn from(s: &str) -> Self {
        let lines = input_as_lines(s);
        let mut map = NorthPoleMap::default();
        let mut guard = NorthPoleGuard::default();

        map.height = lines.len() as i32;
        map.width = if lines.is_empty() { 0 } else { lines[0].len() } as i32;
        for (ey, line) in lines.iter().enumerate() {
            let y = ey as i32;
            for (ex, c) in line.chars().enumerate() {
                let x = ex as i32;
                if c == '^' {
                    guard.pos = Point32 { x, y };
                }
                map.chars.insert(Point32 { x, y }, match c {
                    '^' => '.',
                    _ => c,
                });
            }
        }

        NorthPoleLab { map, guard, }
    }
}

fn default_input() -> &'static str {
    include_input!(06)
}

pub fn part1() -> String {
    let lab = NorthPoleLab::from(default_input());
    lab.walk_part1().to_string()
}

pub fn part2() -> String {
    let lab = NorthPoleLab::from(default_input());
    lab.walk_part2().to_string()
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
        let input = "....#.....\r\n.........#\r\n..........\r\n..#.......\r\n.......#..\r\n..........\r\n.#..^.....\r\n........#.\r\n#.........\r\n......#...";

        let lab = NorthPoleLab::from(input);
        assert_eq!(lab.map.width, 10);
        assert_eq!(lab.map.height, 10);
        assert_eq!(lab.guard.pos.x, 4);
        assert_eq!(lab.guard.pos.y, 6);
        assert_eq!(lab.map.chars[&Point32 { x: 0, y: 0 }], '.');
        assert_eq!(lab.map.chars[&Point32 { x: 4, y: 0 }], '#');

        let walk = lab.walk_part1();
        assert_eq!(walk, 41);
    }

    #[test]
    fn walk_part1() {
        let input = "....\r\n.#..\r\n.^..\r\n....";

        let lab = NorthPoleLab::from(input);
        let walk = lab.walk_part1();
        assert_eq!(walk, 3);
    }

    #[test]
    fn walk_part2() {
        let input = ".#......\r\n.......#\r\n...#....\r\n.....#..\r\no.o^....\r\n....#...\r\n#.......\r\n......#.\r\n";

        let lab = NorthPoleLab::from(input);
        let walk = lab.walk_part2();
        assert_eq!(walk, 2);
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "5312");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "1748");
    }
}
