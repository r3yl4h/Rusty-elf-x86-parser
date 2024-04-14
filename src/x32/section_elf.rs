use std::fs::File;
use std::{io, mem};
use std::io::{Read, Seek, SeekFrom};
use crate::x32::header_elf::ElfHeader32;
use crate::x32::symtab;


#[repr(C)]
#[derive(Clone, Default)]
pub struct SectionHeader32 {
    pub name: u32,
    pub s_type: u32,
    pub flags: u32,
    pub l_addr: u32,
    pub offset: u32,
    pub size: u32,
    pub link: u32,
    pub info: u32,
    pub align: u32,
    pub entsize: u32,
}


fn print_h32(sh_table: &[SectionHeader32], sh_str: &[u8], file: &mut File) {
    println!("======================================================================================================================================");

    for (i, sh) in sh_table.iter().enumerate() {
        println!("\n-----------------------------------");
        println!(" SEH: {i}");
        println!("    Name: {}", crate::x64::section_elf::get_section_name(sh.name as usize, sh_str));
        println!("    offset: {:#x} - load-addr: {:#x}", sh.offset, sh.l_addr);
        println!("    size: {:#x}", sh.size);
        println!("    algn: {}", sh.align);
        println!("    flag: {} - {}", sh.flags, crate::x64::section_elf::get_flag_description(sh.flags as u64));
        println!("    types: {:#x} - {}", sh.s_type, crate::x64::section_elf::get_type_description(sh.s_type));
        let mut data_section = vec![0u8; sh.size as usize];
        file.seek(SeekFrom::Start(sh.offset as u64)).unwrap();
        file.read_exact(&mut data_section).unwrap();
    }
    println!("\n========================================================================================================================");
}




pub fn pars_section(file: &mut File, elf_header: ElfHeader32) -> Result<(), io::Error>{
    let mut section_header = vec![SectionHeader32::default(); elf_header.sh_num as usize];
    file.seek(SeekFrom::Start(elf_header.sh_offset as u64))?;
    for i in 0..elf_header.sh_num {
        file.read_exact(unsafe {
            std::slice::from_raw_parts_mut(&mut section_header[i as usize] as *mut SectionHeader32 as *mut u8, mem::size_of::<SectionHeader32>())
        })?;
    }
    let mut section_str = Vec::new();
    let sh_str_index = elf_header.shstrndx as usize;
    if sh_str_index < elf_header.sh_num as usize {
        let sh_str_offset = section_header[sh_str_index].offset as usize;
        file.seek(SeekFrom::Start(sh_str_offset as u64))?;
        file.read_to_end(&mut section_str)?;
        print_h32(&section_header, &section_str, file);
    } else {
        return Err(io::Error::new(io::ErrorKind::Other, "Invalid section header string table index"))
    }
    Ok(symtab::parse_symb(file, section_header)?)
}