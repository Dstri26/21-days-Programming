/*

DAY 4:
Given a time in 12-hour AM/PM format, convert it to military (24-hour) time.

Details:
The program can be used to convert a time in 12-hour AM/PM format to military(24-hour) time.

Inputs:
Enter a time in 12-hour AM/PM format. 

Outputs:
Convert the input time to military time and print it on the screen.

*/

use std::io;
fn main() {
    let mut time = String::new();
    io::stdin().read_line(&mut time).expect("Error");
    if time.contains("PM"){
        let temp_time:&str = &time[0..8];
        let mut result : Vec<u64> = temp_time.split(":").map(|x| x.parse::<u64>().unwrap()).collect();
        result[0]= result[0] + 12;
        println!("{}:{}:{} PM",result[0],result[1],result[2]);
    }
    else if time.contains("AM"){
        println!("{}",time);
    }
    
    
}
