use std::fs::{File, read};
use std::error::Error;
use elf;
use std::path::PathBuf;

pub fn read_elf(bytes: &[u8]) {
    match goblin::elf::Elf::parse(&bytes) {
        Ok(binary) => {
            println!("{:#?}", binary);
            let entry = binary.entry;
            for ph in binary.section_headers {
                println!("Section: {} {}", ph.sh_type, ph.sh_name);
                // if ph.p_type == goblin::elf::program_header::PT_LOAD {
                //     println!("Found PT_LOAD")
                //     // TODO: you should validate p_filesz before allocating.
                //     //let mut _buf = vec![0u8; ph.p_filesz as usize];
                //     // read responsibly
                // } else {
                //     println!("Some other stuff: {}", ph.p_type);
                // }
            }
        },
        Err(_) => ()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    //let bytes = read("/Users/roman/rust/blinky-f051/target/thumbv6m-none-eabi/release/blinky-f051")?;

    //read_elf(bytes.as_ref());

    let path = PathBuf::from("/Users/roman/rust/blinky-f051/target/thumbv6m-none-eabi/release/blinky-f051");
    let file = match elf::File::open_path(&path) {
        Ok(f) => f,
        Err(e) => panic!("Error: {:?}", e),
    };

    for s in &file.sections {
        println!("{}", s.shdr.name)
    }

    let text_scn = match file.get_section(".picofmt") {
        Some(s) => s,
        None => panic!("Failed to look up .text section"),
    };

    println!("{:?}", text_scn.data);

    Ok(())
}
