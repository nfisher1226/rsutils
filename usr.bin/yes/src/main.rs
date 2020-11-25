use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut s = String::from("y");
    if args.len() > 1 {
        s = args[1].to_string();
    }
    while 1 < 2 {
        println!("{}", s);
    }
}
