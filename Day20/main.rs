/*
DAY 20:

Write a program to find the number of possible combinations of the elements in an array,without repetition, 
 such that their sum is always lesser than or equal to a number x. 

Details:
Given an array of n number of elements, find all the possible combinations of the elements of an array, 
which on being added, shall result in a sum less than or equal to x.

Inputs:
l = length of the array
a = array elements separated by a space
x = the number which shall act as the condition

Outputs:
Number of such possible combinations. 
If no such combinations are possible, print -1

*/
use std::io;
static mut COUNT:i64=0;
fn pcomb(result : Vec<u64>, n:usize, x:usize,rsum:u64) 
{ 
    let data:Vec<u64>=vec![0; n];
    cutil(result.clone(), n, x, 0, data, 0,rsum);
} 
fn cutil(result:Vec<u64>, n:usize, x:usize, index:usize,mut data:Vec<u64>,i:usize,rsum:u64){
    if index == x { 
        let mut sum=0;
        for j in 0..x{
            sum=sum+data[j];
        }
        if sum<=rsum{
            unsafe{
                COUNT=COUNT+1;
            }
        } 
        return;            
    }
    if i >= n{
        return;
    } 
    data[index] = result[i]; 
    cutil(result.clone(), n, x, index + 1, data.clone(), i + 1,rsum); 
    cutil(result.clone(), n, x, index, data.clone(), i + 1,rsum);
}
fn main(){
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Error");

    let mut rsum = String::new();
    io::stdin().read_line(&mut rsum).expect("Error");
    let rsum: u64 = rsum.trim().parse().expect("Error");

    let mut result : Vec<u64> = num.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
    result.sort();
    for r in 1..result.len(){
        pcomb(result.clone(), result.len(),r,rsum);
    }
    unsafe{
        println!("{}",COUNT);
    }
}
    
