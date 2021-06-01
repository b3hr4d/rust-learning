pub fn run() {
    println!("Max for i32: {}", std::i32::MAX);
    println!("Max for i64: {}", std::i64::MAX);

    let is_greater = 10 > 5;

    let face = '\u{1F600}';

    println!("{:?}", (is_greater, face));
}
