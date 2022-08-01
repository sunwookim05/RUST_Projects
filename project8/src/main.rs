use std::io;
 
fn pibo(i: i128, x: i128, y: i128) -> i128{
    if i <= 2{
        return x + y
    }
    pibo(i - 1, y, x + y)
}

fn main() {
    let mut i = String::new();

    io::stdin().read_line(&mut i).expect("error");
    let i: i128 = i.trim().parse().expect("error");

    println!("{}",pibo(i, 0, 1));

    return;
}