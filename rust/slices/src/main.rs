fn print(x: &str) {
    println!("{}", x);
}

fn print_vector(xs: &Vec<i32>) {
    println!("{:?}", xs);
}

fn print_array(xs: &[i32]) {
    println!("{:?}", xs);
}

fn main() {
    let a = String::from("test");
    print(a.as_str());

    let mut b = "hello world";
    print(b);

    b = "new hello world";
    print(b);

    print(&b[1..5]);

    let vector = vec![1,2,3];
    print_vector(&vector);
    print_array(&vector[1..2]);
}
