use std::io;

fn sqrt(x:i64) -> i64{
    let mut result:i64 = x;
    result = (result + (x / result)) / 2;
    result
}

fn main() {
    let mut powin = String::new();
    io::stdin().read_line(&mut powin) .expect("Failed");
    let a:Vec<i64> = powin.split_whitespace().map(|s|s.trim().parse().expect("Failed")).collect::<Vec<_>>();

    let result:i64 = sqrt(a[0]);
    println!("{}",result);
}