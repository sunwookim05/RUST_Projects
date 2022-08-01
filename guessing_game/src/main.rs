use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let secret_num = rand::thread_rng().gen_range(1, 101);

    println!("���ڸ� ���纾�ô�!");
    for _i in 0..10{
        let mut guess = String::new();
        println!("�����̶�� �����ϴ� ���ڸ� �Է��ϼ���.");
        io::stdin().read_line(&mut guess).expect("�Է��� ���� ���� ���߽��ϴ�.");
        let guess: i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("�Է��� ��: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Up"),
            Ordering::Greater => println!("Down"),
            Ordering::Equal => {
                println!("����!");
                return;
            }
        }
    }
    println!("����!");

    return;
}
