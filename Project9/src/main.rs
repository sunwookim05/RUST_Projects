use std::io;
use std::fmt::Write;

fn change_str(st: &mut String, s: String){
    write!(st,"{}",s);
}

fn main() {
    let mut st = String::from("");

    for _i in 0..100{
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("val error");
        let s: String = s.trim().parse().expect("val error");
        if s == "stop"{
            print!("{}",st);
            break;
        }
        change_str(&mut st, s);
    }

    return;
}
