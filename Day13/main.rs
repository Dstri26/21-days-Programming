/*

DAY 13:
Given a positive integer n( 1 <= n <= 1015). Find the largest prime factor of a number.	

Details:
The program can be used to find the largest prime factor of a number.

Inputs:
Input any number.

Outputs:
Print largest prime factor of the number.

*/

#[macro_use]
extern crate text_io;

fn prime_check(num: i64) -> bool {
    let mut k: i64 = 0;
    for i in 2..num {
        if num % i == 0 {
            k = -1;
        }
    }
    if k == -1 {
        false
    } else {
        true
    }
}

fn main() {
    let num: i64 = read!();
    for i in 2..num{
        if num%i==0{
            if prime_check(num/i) {
                println!("{} ", num/i);
                break;
            }
        }   
    }
}
