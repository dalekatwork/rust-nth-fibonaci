use failure::Fallible;

fn main() {
    println!("Fibonacci series");
    loop {
        println!("Enter position for Fibo series");
        match read_number() {
            Ok(n) => println!("the {}th fibo number is {}", n, fibo(n)),
            Err(e) => println!("error: {}", e),
        }
    }
}

fn read_number() -> Fallible<u32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    let n = line.trim().parse::<u32>()?;
    Ok(n)
}

fn fibo(n: u32) -> u32 {
    if n <= 1 {
        0
    } else {
        let mut a = 0;
        let mut b = 1;
        let mut c = 1;
        for _ in 1..n - 1 {
            c = a + b;
            a = b;
            b = c;
        }
        c
    }
}
