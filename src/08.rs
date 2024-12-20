
use advent::*;
use itertools::Itertools;
use std::collections::{HashSet, HashMap};

#[derive(Debug, Default, Clone)]
struct AntennaMap {
    chars: HashMap<Point32, char>,
    width: i32,
    height: i32,
}

impl AntennaMap {
    fn valid_pos(&self, p: Point32) -> bool {
        p.x >= 0 && p.x < self.width && p.y >= 0 && p.y < self.height
    }

    fn all_freq(&self) -> Vec<char> {
        self.chars.values()
            .filter(|c| **c != '.')
            .copied()
            .unique()
            .sorted()
            .collect()
    }

    fn find_freq(&self, ch: char) -> HashSet<Point32> {
        self.chars.iter().filter(|(_, c)| **c == ch).map(|(p, _)| *p).collect()
    }

    fn find_antinodes(&self, ch: char, iter: u16) -> HashSet<Point32> {
        let mut antinodes = HashSet::new();

        let freq = self.find_freq(ch);

        let freq_comb = freq.iter().combinations(2);

        for comb in freq_comb {
            let f1 = comb[0];
            let f2 = comb[1];
            let dx = f2.x - f1.x;
            let dy = f2.y - f1.y;

            let mut p1_iter = iter;
            let mut p1 = Point32 { x: f1.x - dx, y: f1.y - dy };
            while p1_iter > 0 && self.valid_pos(p1) {
                antinodes.insert(p1);
                p1 = Point32 { x: p1.x - dx, y: p1.y - dy };
                p1_iter -= 1;
            }

            let mut p2_iter = iter;
            let mut p2 = Point32 { x: f2.x + dx, y: f2.y + dy };
            while p2_iter > 0 && self.valid_pos(p2) {
                antinodes.insert(p2);
                p2 = Point32 { x: p2.x + dx, y: p2.y + dy };
                p2_iter -= 1;
            }
            if iter > 1 {
                antinodes.insert(*f1);
                antinodes.insert(*f2);
            }

            //println!("f1=[{},{}]  f2=[{},{}]  d=[{},{}]", f1.x, f1.y, f2.x, f2.y, dx, dy);
            //println!("p1=[{},{}]  p2=[{},{}]", p1.x, p1.y, p2.x, p2.y);
            //println!("====");
        }

        antinodes
    }
}

impl From<&str> for AntennaMap {
    fn from(s: &str) -> Self {
        let lines = input_as_lines(s);
        let mut map = AntennaMap::default();

        for (ey, line) in lines.iter().enumerate() {
            let y = ey as i32;
            for (ex, c) in line.chars().enumerate() {
                let x = ex as i32;
                map.chars.insert(Point32 { x, y }, c);
            }
        }

        map.width = if lines.is_empty() { 0 } else { lines[0].len() } as i32;
        map.height = lines.len() as i32;

        map
    }
}

fn default_input() -> &'static str {
    include_input!(08)
}

pub fn part1() -> String {
    let amap = AntennaMap::from(default_input());

    let mut hset = HashSet::new();

    for freq in amap.all_freq() {
        for anti in amap.find_antinodes(freq, 1) {
            hset.insert(anti);
        }
    }

    hset.len().to_string()
}

pub fn part2() -> String {
    let amap = AntennaMap::from(default_input());

    let mut hset = HashSet::new();

    for freq in amap.all_freq() {
        for anti in amap.find_antinodes(freq, 1000) {
            hset.insert(anti);
        }
    }

    hset.len().to_string()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_pos() {
        let amap = AntennaMap::from(include_input!(08test1));
        assert_eq!(amap.width, 10);
        assert_eq!(amap.height, 10);
        assert!(amap.valid_pos(Point32 { x: 0, y: 0 }));
        assert!(amap.valid_pos(Point32 { x: 1, y: 0 }));
        assert!(amap.valid_pos(Point32 { x: 0, y: 1 }));
        assert!(!amap.valid_pos(Point32 { x: -1, y: 0 }));
        assert!(!amap.valid_pos(Point32 { x: 0, y: -1 }));
        assert!(amap.valid_pos(Point32 { x: 9, y: 0 }));
        assert!(amap.valid_pos(Point32 { x: 0, y: 9 }));
        assert!(!amap.valid_pos(Point32 { x: 10, y: 0 }));
        assert!(!amap.valid_pos(Point32 { x: 0, y: 10 }));
    }

    #[test]
    fn all_freq() {
        let amap = AntennaMap::from(include_input!(08test1));
        assert_eq!(amap.all_freq(), vec!['#','a']);
    }

    #[test]    
    fn find_freq() {
        let amap = AntennaMap::from(include_input!(08test1));
        let freq = amap.find_freq('a');
        assert_eq!(freq.len(), 3);
        assert!(freq.contains(&Point32 { x: 4, y: 3 }));
        assert!(freq.contains(&Point32 { x: 8, y: 4 }));
        assert!(freq.contains(&Point32 { x: 5, y: 5 }));
    }

    #[test]
    fn find_antinodes_08test1() {
        let amap = AntennaMap::from(include_input!(08test1));
        let antinodes = amap.find_antinodes('a', 1);
        assert_eq!(antinodes.len(), 4);
        assert!(antinodes.contains(&Point32 { x: 3, y: 1 }));
        assert!(antinodes.contains(&Point32 { x: 0, y: 2 }));
        assert!(antinodes.contains(&Point32 { x: 6, y: 7 }));
        assert!(antinodes.contains(&Point32 { x: 2, y: 6 }));
    }

    #[test]
    fn find_antinodes_08test2() {
        let amap = AntennaMap::from(include_input!(08test2));
        let antinodes = amap.find_antinodes('T', 1000);
        assert_eq!(antinodes.len(), 9);
        assert!(antinodes.contains(&Point32 { x: 5, y: 0 }));
        assert!(antinodes.contains(&Point32 { x: 6, y: 2 }));
        assert!(antinodes.contains(&Point32 { x: 9, y: 3 }));
        assert!(antinodes.contains(&Point32 { x: 2, y: 4 }));
        assert!(antinodes.contains(&Point32 { x: 3, y: 6 }));
        assert!(antinodes.contains(&Point32 { x: 4, y: 8 }));
        assert!(antinodes.contains(&Point32 { x: 0, y: 0 }));
        assert!(antinodes.contains(&Point32 { x: 3, y: 1 }));
        assert!(antinodes.contains(&Point32 { x: 1, y: 2 }));
        
    }

    #[test]
    fn find_antinodes_08test3() {
        let amap = AntennaMap::from(include_input!(08test3));
        let mut hset = HashSet::new();

        for freq in amap.all_freq() {
            for anti in amap.find_antinodes(freq, 1000) {
                hset.insert(anti);
            }
        }

        assert_eq!(hset.len(), 34);
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "351");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "1259");
    }
}
