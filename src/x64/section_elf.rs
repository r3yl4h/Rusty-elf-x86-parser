use std::{io, mem};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use crate::x64::header_elf::ElfHeader64;
use crate::x64::symtab;


#[repr(C)]
#[derive(Clone, Default)]
pub struct SectionHeader64 {
    pub name: u32,
    pub s_type: u32,
    pub flags: u64,
    pub l_addr: u64,
    pub offset: u64,
    pub size: u64,
    pub link: u32,
    pub info: u32,
    pub align: u64,
    pub entsize: u64,
}


pub(crate) fn print_h(sh_table: &[SectionHeader64], sh_str: &[u8], file: &mut File) {
    println!("======================================================================================================================================");

    for (i, sh) in sh_table.iter().enumerate() {
        println!("\n-----------------------------------");
        println!(" SEH: {i}");
        println!("    Name: {}", get_section_name(sh.name as usize, sh_str));
        println!("    offset: {:#x} - load-addr: {:#x}", sh.offset, sh.l_addr);
        println!("    size: {:#x}", sh.size);
        println!("    algn: {}", sh.align);
        println!("    flag: {} - {}", sh.flags, get_flag_description(sh.flags));
        println!("    types: {:#x} - {}", sh.s_type, get_type_description(sh.s_type));
        let mut data_section = vec![0u8; sh.size as usize];
        file.seek(SeekFrom::Start(sh.offset)).unwrap();
        file.read_exact(&mut data_section).unwrap();
    }
    println!("\n========================================================================================================================");
}


pub(crate) fn get_flag_description(flag_value: u64) -> &'static str {
    match flag_value {
        0x1 => "SHF_WRITE (writable)",
        0x2 => "SHF_ALLOC (readable)",
        0x4 => "SHF_EXECINSTR (executable)",
        0x10 => "SHF_MERGE",
        0x20 => "SHF_STRINGS",
        0x40 => "SHF_INFO_LINK",
        0x80 => "SHF_LINK_ORDER",
        0x100 => "SHF_OS_NONCONFORMING",
        0x200 => "SHF_GROUP",
        0x400 => "SHF_TLS",
        0x0FF00000 => "SHF_MASKOS",
        0xF0000000 => "SHF_MASKPROC",
        0x4000000 => "SHF_ORDERED",
        0x8000000 => "SHF_EXCLUDE",
        _ => "",
    }
}


pub(crate) fn get_type_description(types: u32) -> &'static str {
    match types {
        0x0 => "SHT_NULL",
        0x1 => "SHT_PROGBITS",
        0x2 => "SHT_SYMTAB (initialized data)",
        0x3 => "SHT_STRTAB (initialized data)",
        0x4 => "SHT_RELA",
        0x5 => "SHT_HASH (initialized data)",
        0x6 => "SHT_DYNAMIC (initialized data)",
        0x7 => "SHT_NOTE",
        0x8 => "SHT_NOBITS (uninitialized data)",
        0x9 => "SHT_REL",
        0x0a => "SHT_SHLIB",
        0x0b => "SHT_DYNSYM (initialized data)",
        _ => "unknown",
    }
}

pub(crate) fn get_section_name(offset: usize, sh_str: &[u8]) -> String {
    sh_str[offset..]
        .iter()
        .take_while(|&&c| c != 0)
        .map(|&c| c as char)
        .collect()
}


pub fn pars_section(file: &mut File, elf_header: ElfHeader64) -> Result<(), io::Error>{
    let mut section_header = vec![SectionHeader64::default(); elf_header.sh_num as usize];
    file.seek(SeekFrom::Start(elf_header.sh_offset))?;
    for i in 0..elf_header.sh_num {
        file.read_exact(unsafe {
            std::slice::from_raw_parts_mut(&mut section_header[i as usize] as *mut SectionHeader64 as *mut u8, mem::size_of::<SectionHeader64>())
        })?;
    }
    let mut section_str = Vec::new();
    let sh_str_index = elf_header.shstrndx as usize;
    if sh_str_index < elf_header.sh_num as usize {
        let sh_str_offset = section_header[sh_str_index].offset as usize;
        file.seek(SeekFrom::Start(sh_str_offset as u64))?;
        file.read_to_end(&mut section_str)?;
        print_h(&section_header, &section_str, file);
    } else {
        return Err(io::Error::new(io::ErrorKind::Other, "Invalid section header string table index"))
    }
    symtab::parse_symb(file, section_header)?;
    Ok(())
}


