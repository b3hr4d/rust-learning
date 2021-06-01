pub fn run() {
    let mut hello = String::from("Hello ");

    println!("{} length is: {}", hello, hello.len());

    hello.push('W');
    hello.push_str("orld");

    println!("{} ,Length is: {}", hello, hello.len());

    println!(
        "{} ,is contains 'World': {}",
        hello,
        hello.contains("World")
    );

    println!(
        "{} After Replace: {}",
        hello,
        hello.replace("World", "There")
    );

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    assert_eq!(11, hello.len());
}
