/*
DAY 4:
Write a program to print all the prime numbers in an interval using function.
Details:
The program can be used to find the prime numbers in between two numbers. Write a function to check whether a number is prime or not.
Inputs:
Input the starting number.
Input the ending number.
Outputs:
Print all the prime numbers in between the two numbers.

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
    let sn: i64 = read!();
    let ln: i64 = read!();

    for i in sn..ln + 1 {
        if prime_check(i) {
            print!("{} ", i);
        }
    }
}
