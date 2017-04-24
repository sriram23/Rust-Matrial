fn main() {
    let arr = [1,2,3,4,5,6,7,8,9];
    println!("The Complete Array: {:?}",arr);
    println!("The First Number is: {}",arr[0]);
    println!("The Length of array is: {}",arr.len());
    println!("The inbetween elements are: {:?}",&arr[3..7]);
}
