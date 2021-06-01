use std::env;

pub fn run() {
    let mut args = env::args().skip(1);
    let command = args.next().unwrap();

    println!("Command is {}", command);

    if command == "write" {
        let key = args.next().expect("The key is not available!");
        let value = args.next().unwrap();
        println!("The Key is {} and Value is {:?}", key, value);
        let contents = format!("{}\t{}\n", key, value);
        let write_resault = std::fs::write("kv.db", contents);
        match write_resault {
            Ok(()) => {
                println!("Success!")
            }
            Err(e) => {
                println!("{}", e)
            }
        }
    } else if command == "read" {
        let dir = args.next().expect("The directory is not available!");
        match std::fs::read(dir) {
            Ok(e) => {
                let s = String::from_utf8(e).expect("Found invalid UTF-8");
                println!("{}", s)
            }
            Err(e) => {
                println!("{}", e)
            }
        }
    } else {
        println!("Available Command ['write', 'read']");
    }
}
