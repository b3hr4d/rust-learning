pub fn run() {
    greeting("Hello", "Behrad");
    let get_sum = add(6, 8);
    println!("Sum: {}", get_sum);

    let n3 = 8;
    let add_num = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C Sum: {}", add_num(3, 6))
}

fn greeting(greet: &str, name: &str) {
    println!("{}, {}", greet, name)
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
