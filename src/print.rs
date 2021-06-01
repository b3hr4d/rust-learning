pub fn run() {
    println!("hello from print");

    println!(
        "{0} likes to play {1}, kolan {0} gave!",
        "bahamin", "football"
    );

    println!(
        "{name} likes to play {activity}",
        name = "bahamin",
        activity = "football"
    );

    println!("Binary: {0:b} Hex: {0:x} Octal: {0:o}", 10);

    println!("{:?}", (12, false, "hi"));

    println!("10 + 10 = {}", 10 + 10)
}
