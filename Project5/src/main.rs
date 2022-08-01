use std::io;

fn main(){

    let mut stin = String::new();
    let mut count = String::new();

    io::stdin().read_line(&mut stin).expect("stin reading error");
    io::stdin().read_line(&mut count).expect("count reading error");

    let count: i32 = count.trim().parse().expect("count error");

    printf(stin, count);

    return;
}

fn printf(st: String, a: i32){

    for _i in 0..a{
        print!("{}",st);
    }

    return;
}