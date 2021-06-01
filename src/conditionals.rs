pub fn run() {
    let age = 20;
    let check_id = false;
    let knows_person = true;

    if age >= 20 && check_id || knows_person {
        println!("Bartende: What would you like to drink?");
    } else if age < 20 && check_id {
        println!("Bartende: Sorry you have to leave!");
    } else {
        println!("Bartende: I'll need to check your ID!");
    }

    let is_of_age = if age >= 20 { true } else { false };
    println!("Is of Age ? {}", is_of_age)
}
