/*

DAY 9:
Given a number n, the task is to find the odd factor sum.

Details:
The program is used to find the sum of all the odd factors for the given number.

Inputs:
Take any number as the input.

Outputs:
Print the sum of all the odd factors of that number and display it on the screen.

*/

#[macro_use]
extern crate text_io;

fn main() {
    let num: i64 = read!();
    let mut sum: i64= 0;
    for i in 1..num {
        if num%i==0 && i%2!=0{
            sum= sum+i;
        }
    }
    println!("{}",sum);
}
