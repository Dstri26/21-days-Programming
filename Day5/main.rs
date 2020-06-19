/*
DAY 5:
Write a program to convert binary number to a character.

Details:
The program can be used to convert binary number to decimal number. Then the decimal number can be converted to the character whose ASCII value is same as the obtained decimal number.

Inputs:
Input a binary number.

Outputs:
Print the respective Character.
*/

#[macro_use]
extern crate text_io;

fn main() {
    let bin_num: i64 = read!();
    let mut dec_num: i64 = 0;
    let mut base: i64 = 1;
    let mut temp_num: i64 = bin_num;
    while temp_num > 0 {
        let ld: i64 = temp_num % 10;
        temp_num = temp_num / 10;
        dec_num += ld * base;
        base = base * 2;
    }
    println!("{}", dec_num as u8 as char);
}
