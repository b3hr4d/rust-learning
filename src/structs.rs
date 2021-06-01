struct Color1 {
    red: u8,
    green: u8,
    blue: u8,
}

//Tuple struct
struct Color2(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    //constract person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
    //get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }
    //name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c1 = Color1 {
        red: 255,
        green: 0,
        blue: 0,
    };

    c1.red = 200;
    println!("Color: {} {} {}", c1.red, c1.green, c1.blue);

    let mut c2 = Color2(200, 0, 0);

    c2.0 = 255;
    println!("Color: {} {} {}", c2.0, c2.1, c2.2);

    let p = Person::new("Behrad", "Deylami");
    println!("Person {}", p.full_name());
    let mut s = Person::new("shahrzad", "Deylami");
    println!("Person {}", s.full_name());
    s.set_last_name("Dehpour");
    println!("Person {}", s.full_name());
    println!("Person Tuple {:?}", s.to_tuple());
}
