use std::io;

fn numplu(_i: i32) -> i32{
    let mut i = _i;
    let mut j;
    let mut r = 0;

    while i > 9{
        j = i;
		j %= 10;
		r += j;
		i /= 10;
    }
    r + i
}

fn main() {
    let mut a = String::new();

    println!("정수입력");
    io::stdin().read_line(&mut a).expect("error");
    let a: i32 = a.trim().parse().expect("error");

    println!("{:5}", numplu(a));

    return;
}