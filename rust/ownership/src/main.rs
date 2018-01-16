
fn str_len(str: &String) -> usize {
    str.len()
}

fn main() {
    let mut x: String = String::from("test");
    x.push_str(" new");

    println!("str = {}, len = {}", x, str_len(&x));
}
