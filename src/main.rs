#![allow(unused)]

use stif_test::SizedField;

use crate::csif::{errors::ParsingError, SizedPointer, io::DynSizedField};
mod csif;

fn example_data() -> Vec<u8> {
    vec![
        0x08, 0x00, 0x00, 0x00,
        0x0E, 0x00, 0x00, 0x00,
        0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x0a
    ]
}

fn main() -> Result<(), ParsingError> {
    let example = example_data();
    
    let v = &[
        "Hello, world!\n".to_string().csif_into(),
        32f32.csif_into(),
        10u8.csif_into()
    ].concat();

    println!("{:#?}", v);

    println!("{:#?}", String::csif_from(v, SizedPointer::new(0, 14)));

    Ok(())
}