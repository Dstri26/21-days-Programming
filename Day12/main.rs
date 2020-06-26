/*
DAY 12:
Given a long integer, we need to find if the difference between sum of odd digits and sum of even digits is 0 or not. 
The indexes start from zero (0 index is for leftmost digit).

Details:
The program can be used to check if the difference between the sum of digits placed in the odd index positions and the sum of the 
digits placed in the even index positions is 0 or not.

Inputs:
Input a long integer value.

Outputs:
Find the difference between the sum of digits placed in the odd index positions and the sum of the digits placed in the even index positions 
and find if the difference between the two and if it’s 0 print YES or print NO.

*/

#[macro_use]
extern crate text_io;

fn main() {
    let mut num: i64 = read!();
    let mut odd_sum: i64 = 0;
    let mut even_sum: i64 = 0;
    let numstr = num.to_string();
    for i in 0..numstr.len() {
        let ld: i64 = num % 10;
        if i%2==0{
            even_sum=even_sum+ld;
        }
        else {
            odd_sum = odd_sum+ld;
        }
        num = num / 10;
    }
    if odd_sum == even_sum{
        println!("YES");
    }
    else{
        println!("NO");
    }
    
    
}
