/*
DAY 18:
Write a program to count the minimum  number of steps to make a number divisible by another number.

Details:
Given 2 numbers, a and b, count the minimum number of steps you need to increment or decrement a to make it divisible by b. 
Each step means you can either add or subtract 1 from a.
That is, a+1 or a-1 only

Inputs:
First line will have the number of test cases, n.
In the next n lines, for each test case, Input two numbers a and b.

Outputs:
N lines showing number of steps for each test case

*/


use std::io;

fn main(){
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Error");
    let n: u64 = n.trim().parse().expect("Error");

    for _ in 0..n{
        let mut d = String::new();
        io::stdin().read_line(&mut d).expect("Error");
        let result: Vec<u64> = d.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
        let (a,b)=(result[0],result[1]);
        if a%b==0{
            println!("0");
        }
        else if a%b>b/2{
            println!("+{}",b-(a%b));
        }
        else if a%b<=b/2{
            println!("-{}",a%b);
        }
        
        
    }
}
