use std::io;

fn main() {
    
    let mut i = String::new();
    let mut j;

    io::stdin().read_line(&mut i).expect("error");
    let i: i32 = i.trim().parse().expect("error");
    let mut r = i;

    j = i -1;
    while j >= 1{
        r *= j;
        j -= 1;
    }
    println!("{}! = {}", i, r);

    return;
}
 