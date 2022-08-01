fn main(){

    let mut a = 1;
    let mut b = 1;
    let mut r;

    print!("{:5} ",a);
    print!("{:5} ",b);
    for _i in 0..8{
        r = a + b;
        print!("{:5} ",r);
        a = b;
        b = r;
    }

    return;
}