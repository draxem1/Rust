/****nth fibonaci number********/
//Takes previous two results (n-2 & n-1) adds them together in the fib sequence
//Created 3/5/2022 5:43 P.M.

use std::io;

fn get_nth() -> u32
{
loop
    {
    println!("Enter a number for n:");
    
    let mut n = String::new(); 
    
    //Reads input
    io::stdin()
    .read_line(&mut n)
    .expect("Failed to read line");
    
    let n: u32 = match n.trim().parse()
    {
        Ok(num) => num,
        Err(_) => continue,
    };
    return n;
    }
}

//Uses recursion to calculate nth result
//uses u128 to allocate more memory reduce overflow
fn calculate_nth(n1:u128, n2:u128, count:u32, n:u32)
{
   // let x = n;
    let a = n2;
    let b = n2 + n1;
    
    if count < n
    {
        calculate_nth(a, b, count+1, n);
    }
    else
    {
        println!("The {}th fibonaci number is {}",n, b);
    }
}


fn main() {
    
    let n = get_nth();
    
    if n == 0 || n == 1
    {
        println!("The {}th fibonaci number is {}",n,n);
    }
    else
    {
        calculate_nth(0, 1, 2, n);
    }
}