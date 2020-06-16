/*
Write a Program to display the pattern like pyramid using the alphabet.
 
Details:
The program is used to print the above pattern using multiple loops.

*/


#[macro_use]
extern crate text_io;

fn main() {
    let n: i64 = read!();
    let comp = 65;
    let mut temp: i64=0;
    for i in 1..n + 1 {
        for _j in 0..n - i {
            print!("   ");
        }
        for j in 0..i {
            temp = comp + j;
            print!(" {} ", temp as u8 as char);
        }
        for _j in 1..i {
            temp = temp - 1;
            print!(" {} ", temp as u8 as char);
        }

        println!("");
    }
}
