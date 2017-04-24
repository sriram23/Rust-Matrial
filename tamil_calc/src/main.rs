use std::io;
use std::{u32};

fn main() {
    let mut num1 = String::new();
    println!("முதல் எண்ணை உள்ளிடவும்:");
    io::stdin().read_line(&mut num1).expect("ஏதோ தவறு நடந்துவிட்டது! படிக்க முடியவில்லை ..!");
    let mut num2 = String::new();
    println!("இரண்டாம் எண்ணை உள்ளிடவும்:");
    io::stdin().read_line(&mut num2).expect("ஏதோ தவறு நடந்துவிட்டது! படிக்க முடியவில்லை ..!");
    let n1: u32 = num1.trim().parse().ok().expect("சரியான முழு எண்ணை உள்ளிடவும்");
    let n2: u32 = num2.trim().parse().ok().expect("சரியான முழு எண்ணை உள்ளிடவும்");
    println!("கூட்டுத்தொகை: {}",n1+n2);
    println!("வித்தியாசம்: {}",n1-n2);
    println!("பெருக்கல்: {}",n1*n2);
    if n1>n2{
        println!("வகுத்தல்: {}",n1/n2);
    }
    else{
        println!("வகுத்தல் இந்த வழக்கில் செயல்படுத்த முடியவில்லை.");
    }
}
