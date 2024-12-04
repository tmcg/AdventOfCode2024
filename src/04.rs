
use advent::*;

struct WordSearch {
    puzzle: Board2D<char>,
}

impl From<&str> for WordSearch {
    fn from(s: &str) -> Self {
        let lines = input_as_lines(s);
        let rows = lines.len();
        let cols = lines[0].len();
        let vec = lines.iter().flat_map(|x| x.chars()).collect();

        WordSearch {
            puzzle: Board2D::new(vec, rows, cols),
        }
    }
}

impl WordSearch {
    fn find_xmas_part1(&self) -> Vec<(i64, i64, Compass)> {
        let mut result = vec![];
        let w = "XMAS";

        for i in 0..self.puzzle.height() {
            let y = i as i64;
            for j in 0..self.puzzle.width() {
                let x = j as i64;
                if self.test_word(x, y, Compass::North, w) { result.push((x, y, Compass::North)); }
                if self.test_word(x, y, Compass::NorthEast, w) { result.push((x, y, Compass::NorthEast)); }
                if self.test_word(x, y, Compass::East, w) { result.push((x, y, Compass::East)); }
                if self.test_word(x, y, Compass::SouthEast, w) { result.push((x, y, Compass::SouthEast)); }
                if self.test_word(x, y, Compass::South, w) { result.push((x, y, Compass::South)); }
                if self.test_word(x, y, Compass::SouthWest, w) { result.push((x, y, Compass::SouthWest)); }
                if self.test_word(x, y, Compass::West, w) { result.push((x, y, Compass::West)); }
                if self.test_word(x, y, Compass::NorthWest, w) { result.push((x, y, Compass::NorthWest)); }
            }
        }

        result
    }

    fn find_xmas_part2(&self) -> Vec<(i64, i64)> {
        let mut result = vec![];
        let w = "MAS";

        for i in 0..self.puzzle.height() {
            let y = i as i64;
            for j in 0..self.puzzle.width() {
                let x = j as i64;
                let t1 = self.test_word(x, y, Compass::SouthEast, w) || self.test_word(x + 2, y + 2, Compass::NorthWest, w);
                let t2 = self.test_word(x + 2, y, Compass::SouthWest, w) || self.test_word(x, y + 2, Compass::NorthEast, w);

                if t1 && t2 {
                    result.push((x, y));
                }
            }
        }

        result
    }

    fn test_word(&self, x: i64, y: i64, dir: Compass, word: &str) -> bool {
        if word.is_empty() { return true; }

        let tc = word.chars().next().unwrap();
        if let Some(pc) = self.puzzle.index(x, y) {
            if tc == *pc {
                if word.len() == 1 { return true; }
                
                let next = &word[1..];
                return match dir {
                    Compass::North => self.test_word(x, y - 1, dir, next),
                    Compass::NorthEast => self.test_word(x + 1, y - 1, dir, next),
                    Compass::East => self.test_word(x + 1, y, dir, next),
                    Compass::SouthEast => self.test_word(x + 1, y + 1, dir, next),
                    Compass::South => self.test_word(x, y + 1, dir, next),
                    Compass::SouthWest => self.test_word(x - 1, y + 1, dir, next),
                    Compass::West => self.test_word(x - 1, y, dir, next),
                    Compass::NorthWest => self.test_word(x - 1, y - 1, dir, next),
                };
            }
        } 

        false
    }
}

fn default_input() -> &'static str {
    include_input!(04)
}

pub fn part1() -> String {
    let model = WordSearch::from(default_input());

    model.find_xmas_part1().len().to_string()
}

pub fn part2() -> String {
    let model = WordSearch::from(default_input());
    model.find_xmas_part2().len().to_string()
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
        let ws = WordSearch::from("abc\r\ndef\r\nghi");

        assert_eq!(ws.puzzle.width(), 3);
        assert_eq!(ws.puzzle.height(), 3);
        assert!(ws.puzzle.index(-1, -1).is_none());
        assert_eq!(ws.puzzle.index(0, 0), Some(&'a'));
        assert_eq!(ws.puzzle.index(1, 0), Some(&'b'));
        assert_eq!(ws.puzzle.index(2, 0), Some(&'c'));
        assert_eq!(ws.puzzle.index(0, 1), Some(&'d'));
        assert_eq!(ws.puzzle.index(1, 1), Some(&'e'));
        assert_eq!(ws.puzzle.index(2, 1), Some(&'f'));
        assert_eq!(ws.puzzle.index(0, 2), Some(&'g'));
        assert_eq!(ws.puzzle.index(1, 2), Some(&'h'));
        assert_eq!(ws.puzzle.index(2, 2), Some(&'i'));
        assert!(ws.puzzle.index(0, 3).is_none());
        assert!(ws.puzzle.index(3, 0).is_none());
    }

    #[test]
    fn parse_input() {
        let ws = WordSearch::from(default_input());

        assert_eq!(ws.puzzle.width(), 140);
        assert_eq!(ws.puzzle.height(), 140);
    }

    #[test]
    fn find_xmas_part1() {
        let ws = WordSearch::from("..X...\r\n.SAMX.\r\n.A..A.\r\nXMAS.S\r\n.X....");

        assert_eq!(ws.puzzle.width(), 6);
        assert_eq!(ws.puzzle.height(), 5);
        assert!(ws.puzzle.index(-1, -1).is_none());
        assert!(ws.puzzle.index(0, 0).is_some_and(|c| *c == '.'));
        assert!(ws.puzzle.index(1, 0).is_some_and(|c| *c == '.'));
        assert!(ws.puzzle.index(2, 0).is_some_and(|c| *c == 'X'));
        assert!(ws.puzzle.index(0, 1).is_some_and(|c| *c == '.'));
        assert!(ws.puzzle.index(1, 1).is_some_and(|c| *c == 'S'));
        assert!(ws.puzzle.index(2, 1).is_some_and(|c| *c == 'A'));
        assert!(ws.puzzle.index(0, 5).is_none());
        assert!(ws.puzzle.index(6, 0).is_none());

        let found = ws.find_xmas_part1();

        assert_eq!(found.len(), 4);
        assert_eq!(found[0], (2, 0, Compass::SouthEast));
        assert_eq!(found[1], (4, 1, Compass::West));
        assert_eq!(found[2], (0, 3, Compass::East));
        assert_eq!(found[3], (1, 4, Compass::North));
    }

    #[test]
    fn find_xmas_part2() {
        let w1 = WordSearch::from("M.S\r\n.A.\r\nM.S");

        assert!(w1.test_word(0, 0, Compass::SouthEast, "MAS"));
        assert!(!w1.test_word(2, 0, Compass::SouthWest, "MAS"));
        assert!(w1.test_word(0, 2, Compass::NorthEast, "MAS"));
        assert!(!w1.test_word(2, 2, Compass::NorthWest, "MAS"));

        let f1 = w1.find_xmas_part2();
        assert_eq!(f1.len(), 1);
        assert_eq!(f1[0], (0, 0));
        
        let w2 = WordSearch::from(".M.S......\r\n..A..MSMS.\r\n.M.S.MAA..\r\n..A.ASMSM.\r\n.M.S.M....\r\n..........\r\nS.S.S.S.S.\r\n.A.A.A.A..\r\nM.M.M.M.M.\r\n..........");

        let f2 = w2.find_xmas_part2();
        assert_eq!(f2.len(), 9);
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "2536");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "1875");
    }
}
