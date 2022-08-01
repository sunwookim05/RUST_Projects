use std::io;

fn pow(x:i64, y:i64) -> i64{
    let mut result:i64 = x;
    if x == 1{
        return 1
    }
    for _i in 1..y{
        result *= x;
    }
    if y == 0{
        if x == 0{
            return 0
        }else{
            return 1
        }
    }
    result
}

fn main() {
    let mut powin = String::new();
    io::stdin().read_line(&mut powin) .expect("Failed");
    let a:Vec<i64> = powin.split_whitespace().map(|s|s.trim().parse().expect("Failed")).collect::<Vec<_>>();

    let result:i64 = pow(a[0], a[1]);
    println!("{}",result);
}
