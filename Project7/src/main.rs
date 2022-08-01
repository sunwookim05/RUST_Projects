use std::io;

fn factorial(i: i32) -> i32{
    if i <= 1{
        return 1
    }
    i * factorial(i - 1)
}

fn main() {
    let mut i = String::new();
    io::stdin().read_line(&mut i).expect("error");
    let i: i32 = i.trim().parse().expect("error");

    println!("{}", factorial(i));

    return;
}
