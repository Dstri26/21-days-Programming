/*
DAY 17:
Write a program which uses a recursive function to print a Fibonacci series.

Details:
The program will use a recursive function to print the Fibonacci series. The function will have the following parameters as input:-
a-     First element of the series
b-     Second element of the series
n-     number of elements of the series you wanna display
The program should take user input of the above parameters and pass it to the function for execution.
You can add more parameters if you want, but the above 3 are required for certainity.

Inputs:
Input the starting element a, second element b, and the number of the elements you wanna display.

Outputs:
Print the series till the nth element.
*/


#[macro_use]
extern crate text_io;

fn fibo(fnum:i64,snum:i64,n:i64)->i64{
    if n == fnum || n == snum{
        n
    }
    else{
        fibo(fnum,snum,n-1) + fibo(fnum,snum,n-2)
    }
}

fn main() {
    let fnum:i64 = read!();
    let snum:i64 = read!();
    let n:i64= read!();    
    for i in fnum..n+1{
        print!("{} ",fibo(fnum,snum,i));
    }
}
