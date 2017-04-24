use std::io;
use std::{u32};

fn main() {
    let mut num1 = String::new();
    println!("Enter First Number:");
    io::stdin().read_line(&mut num1).expect("Something went Wrong! Unable to read..!");
    let mut num2 = String::new();
    println!("Enter Second Number:");
    io::stdin().read_line(&mut num2).expect("Something went Wrong! Unable to read..!");
    let n1: u32 = num1.trim().parse().ok().expect("Enter a valid Integer");
    let n2: u32 = num2.trim().parse().ok().expect("Enter a valid Integer");
    println!("The Sum is: {}",n1+n2);
    println!("The Difference is: {}",n1-n2);
    println!("The Product is: {}",n1*n2);
    if n1>n2{
        println!("The Division is: {}",n1/n2);
    }
    else{
        println!("Division could not be performed in this case.");
    }
}
