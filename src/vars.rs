pub fn run() {
    let name = "Behrad";
    let mut age = 34;
    println!("My name is {} an i'm {}", name, age);
    age = 35;
    println!("My name is {} an i'm {}", name, age);

    const ID: i32 = 001;
    println!("ID:{}", ID);

    let (my_name, my_age) = ("Behrad", 34);
    println!("{} is {}", my_name, my_age)
}
