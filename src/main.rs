use utils::ReadUTF8;

mod parser;
mod utils;

fn main() {
    let s = String::from("éléphant");

    while let Some(c) = s.get_utf8(0) {
        println!("{}", c.0);
    }
}


