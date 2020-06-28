/*
DAY 14:
Write a program to replace all occurences of a substring in a string.

Details:
Inputs:
Input any string.
Input the substring you want to replace.
Input the string you want to replace it with.

Outputs:
Print the final string after replacing the substring with the string entered by the user.

*/


use std::io;
fn main() {
    let mut orgstr=String::new();
    io::stdin().read_line(&mut orgstr).expect("Error");
    if let Some('\n')=orgstr.chars().next_back() {
        orgstr.pop();
    }
    let mut substr=String::new();
    io::stdin().read_line(&mut substr).expect("Error");
    if let Some('\n')=substr.chars().next_back() {
        substr.pop();
    }
    let mut newstr=String::new();
    io::stdin().read_line(&mut newstr).expect("Error");
    if let Some('\n')=newstr.chars().next_back() {
        newstr.pop();
    }
    orgstr=orgstr.replace(&substr,&newstr);
    println!("{}",orgstr);
}
