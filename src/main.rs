use std::io::{self, BufReader, Read};

use disassembler::Disassembler;

mod disassembler;

fn main() {
    let f = io::stdin();
    let reader = BufReader::new(f);

    let instructions = Disassembler::new(reader.bytes().flatten());
    for i in instructions {
        dbg!(i);
    }
}
