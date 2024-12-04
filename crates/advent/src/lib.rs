
pub mod shared;

#[macro_export]
macro_rules! include_input {
    ($day:literal) => {
        include_str!(concat!("./input/", stringify!($day), ".txt"))
    };
}

pub fn input_as_lines(s: &str) -> Vec<String> {
    s.split("\r\n").map(|x| x.to_owned()).collect::<Vec<_>>()
}

pub fn input_as_ints(s: &str) -> Vec<i64> {
    let input_lines = input_as_lines(s);

    input_lines.iter()
        .map(|s| s.parse::<i64>().expect("Unable to convert line to i64"))
        .collect()
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Compass {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}


#[derive(Debug)]
pub struct Board2D<T> {
    vec: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Board2D<T> {
    pub fn new(vec: Vec<T>, height: usize, width: usize) -> Self {
        assert!(vec.len() == width * height);
        Self { vec, width, height }
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    pub fn index(&self, x: i64, y: i64) -> Option<&T> {
        if y >= 0 && x >= 0 && (y as usize) < self.height && (x as usize) < self.width  {
            Some(&self.vec[((y as usize) * self.width) + (x as usize)])
        } else {
            None
        }
    }
}