/*

DAY 1:
Write a Program to solve Quadratic Equation depending upon the value of Delta.

Details:
The program is used to solve quadratic equation i.e. ax2 + bx + c = 0 depending upon the value of Delta. 
The formula for calculating Delta is d = b*b-4*a*c.

Inputs:
Take the coefficients a, b and c as input.

Outputs:
Print the value of Delta.
Print the Roots.

Sample:

Inputs
5
2
1

Outputs:
-16
(-2 + 4i)/10 
(-2 – 4i)/10


*/


#[macro_use]
extern crate text_io;

fn main(){
    let a: f64 = read!();
    let b: f64 = read!();
    let c: f64 = read!();
    let delta:f64 = b*b-4.0*a*c;
    println!("{}",delta);
   
    if delta>0.0 {
        //real and different roots
        println!("{}",(delta.abs().sqrt()-b)/(2.0*a));
        println!("{}",(-delta.abs().sqrt()-b)/(2.0*a));
    }
    else if delta==0.0{
        // real and similar roots
        println!("{}",-b/(2.0*a));
        println!("{}",-b/(2.0*a));
    }
    else{
        //imaginary roots
        println!("(-{} + {}i)/{}",b,delta.abs().sqrt(),2.0*a);
        println!("(-{} - {}i)/{}",b,delta.abs().sqrt(),2.0*a);
    }

 }
