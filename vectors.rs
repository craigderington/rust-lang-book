// vectors
use std::mem::size_of_val;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    
    // re-assign value
    numbers[2] = 20;

    // add to vector
    numbers.push(5);

    // remove from vector
    numbers.pop();
    
    // print to stdout
    println!("{:?}", numbers);
    println!("Singleton: {}", numbers[0]);

    // vector loop
    for x in numbers.iter() {
        println!("Number: {}", x);
    }
    
    // mutate value in loop
    for i in numbers.iter_mut() {
        *i *= 2
    }

    // print mutated vector
    print!("Mutated: {:?}", numbers);

    // vector length
    println!("Vector Length: {}", numbers.len());
    
    // memory usage
    println!("Memory Usage: {}", size_of_val(&numbers));

    // get a vector slice
    let slice: &[i32] = &numbers[0..2];
    println!("Vector Slice: {:?}", slice)

}