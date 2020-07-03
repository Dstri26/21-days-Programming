/*
DAY 19:
Find the probability of a perfect square occuring when a number is randomly choosen between numbers a and b over n iterations.

Details:
Given 2 numbers, a and b, you have to find that, over n iterations,
what is the probability that if a number is picked randomly in between a and b, it turns out to be a perfect square.

Inputs:
Input numbers a, b and n.

Outputs:
Print the probability in decimal
*/

#[macro_use]
extern crate text_io;
use rand::Rng;
fn main(){
    let sn:i64 = read!();
    let ln:i64 = read!();
    let tn:i64 = read!();
    let mut sum:i64 = 0;

    for _ in 0..tn{
        let rn = rand::thread_rng().gen_range(sn, ln);
        if (rn as f64).sqrt()==(((rn as f64).sqrt()) as i64) as f64{
            sum=sum+1;
        }        
    }
    println!("{}",sum as f64/tn as f64);
}
