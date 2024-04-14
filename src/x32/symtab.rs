use std::io;
use std::fs::File;
use std::io::{Error, Read, Seek, SeekFrom};
use std::mem;
use crate::x32::section_elf::SectionHeader32;

#[repr(C)]
#[derive(Default)]
struct Elf32Sym {
    st_name: u32,
    st_value: u32,
    st_size: u32,
    st_info: u8,
    st_other: u8,
    st_shndx: u16,
}



pub fn parse_symb(file: &mut File, elf_header: Vec<SectionHeader32>) -> Result<(), Error> {
    let mut dynstr_off = 0;
    let mut dynsym_off = 0;
    let mut dynsym_sz = 0;

    for shdr in elf_header {
        match shdr.s_type {
            2 | 3 => {
                if dynstr_off == 0 {
                    println!("string table offset: {:#x} - size: {:#x}", shdr.offset, shdr.size);
                    dynstr_off = shdr.offset as usize;
                }
            }
            11 => {
                dynsym_off = shdr.offset as usize;
                dynsym_sz = shdr.size as usize;
                println!("dynsym table offset: {:#x} - size: {:#x}", shdr.offset, shdr.size);
            }
            _ => {}
        }
    }

    if dynstr_off == 0 {
        return Err(Error::new(io::ErrorKind::Other, "dynstr is empty"))
    }else if dynsym_off == 0 {
        return Err(Error::new(io::ErrorKind::Other, "dynsym is empty"))
    }

    let mut i = 0;
    while i * mem::size_of::<Elf32Sym>() < dynsym_sz {
        let mut sym = Elf32Sym::default();
        let absoffset = dynsym_off + i * mem::size_of::<Elf32Sym>();
        file.seek(SeekFrom::Start(absoffset as u64))?;
        file.read_exact(unsafe { std::slice::from_raw_parts_mut(&mut sym as *mut Elf32Sym as *mut u8, mem::size_of::<Elf32Sym>())})?;
        println!("\n-----------------------------------");
        println!(" SYH: {i}");
        println!("    offset of sym: {:#x}", absoffset);
        println!("    address offset name = {:#x}", sym.st_name);
        if sym.st_name != 0 {
            let mut name_buffer = vec![0u8; 256];
            file.seek(SeekFrom::Start((dynstr_off + sym.st_name as usize) as u64))?;
            file.read_exact(&mut name_buffer)?;
            let name: String = name_buffer.iter().take_while(|&&c| c != 0).map(|&c| c as char).collect();
            println!("    name: ('{name}') - offset: {:#x}", dynstr_off + sym.st_name as usize);
        }
        println!("    st_infos = {:#x}", sym.st_info);
        println!("    st_other = {:#x} - st_shndx = {}", sym.st_other, sym.st_shndx);
        println!("    st_value = {:#x} - st_size = {}", sym.st_value, sym.st_size);
        i += 1;
    }
    println!("\n");
    Ok(())
}