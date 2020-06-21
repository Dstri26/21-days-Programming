/*
DAY 6:
Write a program to find the frequency of each digit in a given integer.

Details:
The program can be used to count the number of digits in a given number.

Inputs:
Input any number.

Outputs:
Print the respective frequency of a digit.

*/

#[macro_use]
extern crate text_io;

fn main() {
    let num: i64 = read!();
    let mut count: [i64; 10] = [0,0,0,0,0,0,0,0,0,0];
    let mut temp_num: i64 = num;
    while temp_num > 0 {
        let ld: i64 = temp_num % 10;
        temp_num = temp_num / 10;
        match ld {
            0 => count[0]+=1,
            1 => count[1]+=1,
            2 => count[2]+=1,
            3 => count[3]+=1,
            4 => count[4]+=1,
            5 => count[5]+=1,
            6 => count[6]+=1,
            7 => count[7]+=1,
            8 => count[8]+=1,
            9 => count[9]+=1,
            _=>println!("A non-digit in present.")
        }
    }
    for i in 0..10{
        println!("The frequency of {} : {}",i,count[i]);
    }
}
