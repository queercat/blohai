mod lex;
mod parse;

use parse::AST;
use std::fs::File;
use std::io::prelude::*;
use std::io;

const WASM_MAGIC: [u8; 4] = [0x00, 0x61, 0x73, 0x6d];
const WASM_VERSION: [u8; 4] = [0x01, 0x00, 0x00, 0x00]; 

fn create_program(value: i32) -> Vec<u8> {
    fn create_section(id: u8, value: u8) -> Vec<u8> {
        let mut section: Vec<u8> = vec![];

        section.push(id);

        match id {
            // type section
            0x01 => {
                section.push(0x05); // size 5
                section.push(0x01); // 1 type 
                section.push(0x60); // func
                section.push(0x00); // 0 params
                section.push(0x01); // 1 result 
                section.push(0x7f); // returns i32
            }

            // function section
            0x03 => { 
                section.push(0x02); // size 2 
                section.push(0x01); // 1 function
                section.push(0x00); // func location 0
            }

            // export section
            0x07 => {
                section.push(0x0a); // size 10 
                section.push(0x01); // 1 export
                section.push(0x06); // export name 6 bytes
                section.push(0x5F); // _
                section.push(0x73); // s
                section.push(0x74); // t
                section.push(0x61); // a
                section.push(0x72); // r
                section.push(0x74); // t
                section.push(0x00); // export type function
                section.push(0x00); // func location 0
            }

            // code section
            0x0a => {
                section.push(0x07);
                section.push(0x01);
                section.push(0x05);
                section.push(0x00);
                section.push(0x41);
                section.push(value);
                section.push(0x0f);
                section.push(0x0b);
            }
            _ => {section.pop();}
        }

        return section;
    }

    let mut program: Vec<u8> = vec![];

    for v in WASM_MAGIC {
        program.push(v);
    }

    for v in WASM_VERSION {
        program.push(v);
    }

    for i in 0..=10 {
        for v in create_section(i, value as u8) {
            program.push(v);
        }
    }

    return program;
}

fn main() -> std::io::Result<()> {
    let mut blowhai_program = String::new();
    io::stdin().read_line(&mut blowhai_program);

    let result = parse::parse(&blowhai_program);
    let mut contents: Vec<u8> = vec![];

    match result {
        AST::NumberLiteral {value} => {contents = create_program(value);}
        _ => {panic!("non-valid AST produced, /shrug");}
    }

    let mut file = File::create("out.wasm")?;
    for b in contents {
        file.write(&[b]);
    }
    Ok(())
}

