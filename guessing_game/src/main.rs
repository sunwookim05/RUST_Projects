use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let secret_num = rand::thread_rng().gen_range(1, 101);

    println!("숫자를 맞춰봅시다!");
    for _i in 0..10{
        let mut guess = String::new();
        println!("정답이라고 생각하는 숫자를 입력하세요.");
        io::stdin().read_line(&mut guess).expect("입력한 값을 읽지 못했습니다.");
        let guess: i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("입력한 값: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Up"),
            Ordering::Greater => println!("Down"),
            Ordering::Equal => {
                println!("정답!");
                return;
            }
        }
    }
    println!("실패!");

    return;
}
