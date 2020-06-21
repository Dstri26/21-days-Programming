/*
DAY 7:
Write a program to check if a given string is palindrome or not.

Details:
The program can be used to reverse a string or verify if it is a palindrome or not.

Inputs:
Input any string.

Outputs:
Print if the string is palindrome or not.

*/


#[macro_use]
extern crate text_io;

fn main(){
    let num:String = read!();
    let tokens:Vec<&str>= num.split("").collect();
    let mut i:usize=1;
    let mut j:usize=num.len() as usize;
    let mut flag=1;
    loop{
        if i>j{
            break;
        }
        else if tokens[i]!=tokens[j]{
            println!("NO");
            flag=-1;
            break;
        }
        i=i+1;
        j=j-1;

    }
    if flag==1{
        println!("YES");
    }
}
