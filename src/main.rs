use std::io;

fn main() {
    println!("Fibonaci series");
    loop{
        let mut n = String::new();
        println!("Enter postion for Fibo series");
        io::stdin().read_line(&mut n)
        .expect("Failed to read line");
        let n: u32 = match n.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
        fibo(n);
    }    
}

fn fibo(n: u32){
    let mut a = 0;
    let mut b = 1;
    let mut c = a+b;

    let result = if n == 1 {
        a
    } else if n == 2{
        b
    } else {
        for _ in 1..n-1 {
            c= a+b;
            a = b;
            b = c;
        }
        c
    };

    println!("the {}th fibo number is {}", n, result);
}