use std::convert::TryFrom; 

fn main(){
    
    let mut a: [[i32; 10]; 10] = [[0; 10]; 10];
    let mut r = 1;

    for i in 0..10{
        for _j in 0..10{
            a[i][_j] = i32::try_from(r).unwrap();
            r += 1;
            print!("{:3} ", a[i][_j]);
        }
        if i == 9{
            break;
        }
        println!();
        println!();
    }
    
    return;
}

