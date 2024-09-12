use std::io;
// Fibonacci Series Number
// F(n) = 1,1,2,3,5,8,13,....(n-1),((n-1)+n)
// where, n >= 1

fn fibo(mut n:u32)->u32{
        let mut current: u32 = 1;
        let mut next: u32 = 1;

        if n<=0 {return 0}
        if n == 1 || n == 2 {return 1}
        if n > 2 {
            while n > 1{
                let sum:u32 = current + next;
                current = next;
                next = sum;
                n -= 1;
            }
            next
            }
        else{
            return 1;
        }
            // else{
            //     return fibo(n-1)+fibo(n-2);
}

fn main(){
    println!("Fibo of nth: What is nth?: n --> ");
    let mut n = String::new();
    io::stdin()
    .read_line(&mut n)
    .expect("Invalid input");
    let n:u32 = n.trim().parse().expect("Not a number. Please type a number");
    let fibo = fibo(n);
println!("Fibo of {n}: {fibo}");
}
