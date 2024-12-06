
use advent::*;
use std::collections::{HashSet, HashMap};

#[derive(Debug, Default)]
struct NorthPoleMap {
    chars: HashMap<Point32, char>,
    width: i32,
    height: i32,
}

#[derive(Debug, Default)]
struct NorthPoleGuard {
    pos: Point32,
    dir: Compass,
}

#[derive(Debug, Default)]
struct NorthPoleLab {
    map: NorthPoleMap,
    guard: NorthPoleGuard
}

impl NorthPoleLab {

    fn walk(&self) -> HashSet::<Point32> {
        let mut pos = self.guard.pos;
        let mut dir = self.guard.dir;
        let mut walk = HashSet::new();
        let ch = &self.map.chars;

        walk.insert(pos);

        loop {
            let mut next = pos;
            match dir {
                Compass::North => next.y -= 1,
                Compass::East => next.x += 1,
                Compass::South => next.y += 1,
                Compass::West => next.x -= 1,
                _ => (),
            }

            if ch.contains_key(&next) {
                //println!("{:?}: ch={} (dir={:?})", next, ch[&next], dir);
                if ch[&next] == '#' {
                    dir = dir.cardinal_right();
                } else {
                    walk.insert(next);
                    pos = next;
                }

                continue;
            }

            break;
        }

        walk
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
    lab.walk().len().to_string()
}

pub fn part2() -> String {
    //let lab = NorthPoleLab::from(default_input());
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
        let input = "....#.....\r\n.........#\r\n..........\r\n..#.......\r\n.......#..\r\n..........\r\n.#..^.....\r\n........#.\r\n#.........\r\n......#...";

        let lab = NorthPoleLab::from(input);
        assert_eq!(lab.map.width, 10);
        assert_eq!(lab.map.height, 10);
        assert_eq!(lab.guard.pos.x, 4);
        assert_eq!(lab.guard.pos.y, 6);
        assert_eq!(lab.map.chars[&Point32 { x: 0, y: 0 }], '.');
        assert_eq!(lab.map.chars[&Point32 { x: 4, y: 0 }], '#');

        let walk = lab.walk();
        assert_eq!(walk.len(), 41);
    }

    #[test]
    fn walk_impl() {
        let input = "....\r\n.#..\r\n.^..\r\n....";

        let lab = NorthPoleLab::from(input);
        let walk = lab.walk();
        assert_eq!(walk.len(), 3);
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "5312");
    }

    #[test]
    fn solve_part2() {
        //assert_eq!(part2(), "zz");
    }
}
