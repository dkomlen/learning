
fn pow(x: i32, y: i32) -> i32 {
    match (x, y) {
        (_, 0) => 1,
        (_, 1) => x,
        (x, y) if y % 2 == 0 => {
            let z = pow(x, y / 2);
            z * z
        },
        _ => {
            let z = pow(x, (y - 1) / 2);
            x * z * z
        }
    }
}

fn main() {
    for i in 1..10 {
        for j in 1..4 {
            println!("{}^{} = {}", i, j, pow(i,j));
        }
    }
}
