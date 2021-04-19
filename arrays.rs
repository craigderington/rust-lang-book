// arrays, a fixed list where 
// elements are the same data type

use std::mem::size_of_val;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    // re-assign value
    numbers[2] = 20;
    println!("{:?}", numbers);
    println!("Singleton: {}", numbers[0]);
    
    // array length
    println!("Array Length: {}", numbers.len());
    
    // memory usage
    println!("Memory Usage: {}", size_of_val(&numbers));

    // get a slice
    let slice: &[i32] = &numbers[0..2];
    println!("Array Slice: {:?}", slice)

}