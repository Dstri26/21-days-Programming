/*

DAY 15:
Tanya and Shiva have developed a secret way to talk with each other. They use a certain code to encrypt their message. 
Write a simple program to decrypt their hidden message.

Details:
The program is to decrypt a message which is based on the follow rules:-
1 – Every alternate character is replaced by either the previous or the next character as per their ascii values.
2- The 1st  character is replaced by the next ascii value character, the 2nd character by the previous ascii value character, 
the 3rd character again by the next ascii value character, and so on.

Inputs:
Take the encrypted message as input.

Outputs:
Print the hidden message as the output.


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
    let mut i=0;

    for j in orgmsg.chars(){

        if j==' '{
            print!(" ");
            i=i-1;
        }
        else if i%2==0{
            print!("{}",((j as char as u8)-1) as u8 as char);
        }
        else {
            print!("{}",((j as char as u8)+1) as u8 as char);
        }
        i = i + 1;
    }

    
}
