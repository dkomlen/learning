use std::fmt;

#[derive(PartialEq)]
struct Point {
    x: i64,
    y: i64,
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

#[derive(PartialEq, Eq)]
enum Result {
    X,
    Y,
}

fn experimenting_1() {
    let p = Point { x: 3, y: 4 };
    println!("({} {})", p.get_x(), p.get_y());

    let mut x = String::new();
    std::io::stdin().read_line(&mut x).expect("error reading line");
    println!("{}", x);

    let g = Result::X;
    let w = Result::Y;

    if g == g {
        println!("Equal")
    } else {
        println!("Not equal")
    }

    let a = Point { x: 5, y: 10 };
    println!("{}", a == a);
}

fn experimenting_2() {
    let a = 1;
    let mut b = 3;
    b = 4;
    b = a;
}

fn print(x: &str) {
    println!("{}", x);
}

fn main() {
    let a = "43";
    print(a);
    println!("{}", a);

    let s = String::from("test");
    Ok(())?
}
