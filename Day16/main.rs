/*

DAY 16:
Write a program to find the character which appears the most in a string, and then replace all occurrences of this character with the second most frequently appearing character in this string.

Details:
The program should first find the character which appears the most in the string, and then replace the character throughout the string with the next most frequently appearing character.

Inputs:
Input the string for working on.

Outputs:
The output should contain 2 lines:-
First line will be the character which appears the most
Second will be the new string with its most frequent character replaced by the next most frequent character in the string.

*/

use std::io;
fn main() {
    let mut orgmsg=String::new();
    io::stdin().read_line(&mut orgmsg).expect("Error");
    if let Some('\n')=orgmsg.chars().next_back() {
        orgmsg.pop();
    }
    if let Some('\r')=orgmsg.chars().next_back() {
        orgmsg.pop();
    }
    let mut fchar:String='a'.to_string();
    let mut schar:String='b'.to_string();
    let mut fcount =0;
    let mut scount =0;
    
    for j in  orgmsg.chars(){
        let mut count =0;
        for k in orgmsg.chars(){
            if k == j{
                count=count+1;
            }
        }
        if count>=fcount{
            fcount=count;
            fchar = j.to_string();
        }
        else if count > scount{
            scount = count;
            schar =j.to_string();
        }
    }
    orgmsg = orgmsg.replace(&fchar,&schar);
    println!("{}",fchar);
    println!("{}",orgmsg);
    
}
