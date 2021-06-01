use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers);
    numbers[2] = 11;
    numbers[4] = 8;
    println!("{:?}", numbers);

    println!("first number: {}", numbers[0]);

    numbers.push(5);
    numbers.push(15);
    println!("{:?}", numbers);

    println!("Vector length: {}", numbers.len());

    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[1..3];
    println!("Slice is: {:?}", slice);

    for x in numbers.iter() {
        println!("Number: {}", x)
    }

    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Number: {:?}", numbers)
}
