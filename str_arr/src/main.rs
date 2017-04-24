fn main() {
    let str_ar = ["Hello","World!","My","Name","is","RUST"];
    println!("The complete array is: {:?}",str_ar);
    println!("The first string is: {}",str_ar[0]);
    println!("The length of Array is: {}",str_ar.len());
    println!("The inbetween elements are: {:?}",&str_ar[2..6]);
}
