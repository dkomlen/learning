use std::fmt;

struct Point {
    x: i64,
    y: i64
}

impl Point {
    pub fn get_x(&self) -> i64 {
        self.x
    }
    pub fn get_y(&self) -> i64 {
        self.y
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point{ x: 3, y: 4 };
    println!("{}", p);
}
