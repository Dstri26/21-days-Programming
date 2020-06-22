/*

DAY 8:
Given the participants' score sheet for your University Sports Day, you are required to find the runner-up score. You are given  n scores. Store them in a list and find the score of the runner-up.

Details:

Input Format:
The first line contains n, the number of scores.
 The second line contains an array A[]  of n  integers each separated by a space.

Output Format
Print the runner-up score.

*/
use std::io;

fn main() {
    let mut num = String::new();
    let mut s = String::new();
    io::stdin().read_line(&mut num).expect("Error");
    io::stdin().read_line(&mut s).expect("Error");
    let mut result : Vec<u64> = s.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
    result.sort();
    result.reverse();
    let fscore = result[0];
    for i in result{
        if i != fscore{
        println!("{}",i);
        break;
        }
    }

}
