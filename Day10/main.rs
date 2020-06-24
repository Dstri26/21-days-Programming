/*
DAY 10:
Given a list of numbers, write a program to swap first and last element of the list.

Details:
The program can be used to swap the first and last element in a list.

Inputs:
Input a list of n integers.

Outputs:
Print the list with the first and last elements swapped.

*/
use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Error");
    let mut result: Vec<u64> = s
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let num = result.len() - 1;
    let temp = result[0];
    result[0] = result[num];
    result[num] = temp;

    println!("{:?}", result);
}
