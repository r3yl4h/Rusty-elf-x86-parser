use std::fs::File;
use std::io;
use std::io::{Read};
use crate::{x32, x64};



pub fn get_type_file(types: u16) -> &'static str {
    match types {
        1 => "Relocatable file",
        2 => "Executable file",
        3 => "Shared object file",
        4 => "Core file",
        0xfe00 | 0xfeff  => "Operating system-specific",
        0xff00 | 0xffff => "Processor-specific",
        _ => "unknown",
    }
}

pub fn get_machine(machine: u16) -> &'static str {
    match machine {
        3 => "x86-32",
        5 => "Motorola 88000",
        6 => "Reserved for future use was EM_486",
        7 => "Intel 80860",
        8 => "MIPS I",
        9 => "IBM System/370",
        10..=14 => "Reserved for future use",
        15 => "Hewlett-Packard PA-RISC",
        16 => "Reserved for future use",
        17 => "Fujitsu VPP500",
        18 => "Enhanced instruction set SPARC",
        19 => "x32",
        20 => "PowerPC",
        40 => "ARM",
        50 => "IA64",
        62 => "x86-64",
        243 => "RISC-V",
        _ => "unknown",
    }
}


pub(crate) fn type_head(p_type: u32) -> &'static str {
    match p_type {
        0 => "PT_NULL",
        1 => "PT_LOAD",
        2 => "PT_DYNAMIC",
        3 => "PT_INTERP",
        4 => "PT_NOTE",
        5 => "PT_SHLIB",
        6 => "PT_PHDR",
        7 => "PT_TLS",
        0x60000000 => "PT_LOOS",
        0x6474E550 => "EH_FRAME",
        0x6474E551 => "PT_STACK",
        0x6474E552 => "RO-AFTER",
        0x6FFFFFFF => "PT_HIOS",
        0x70000000 => "PT_LOPROC",
        0x7FFFFFFF => "PT_HIPROC",
        _ => "UNKNOW",
    }
}

pub(crate) fn flag_peh_desc(flags: u32) -> &'static str{
    match flags {
        1 => "PF_X (Execute)",
        2 => "PF_W (Write)",
        3 => "PF_W + PF_X (Write, execute)",
        4 => "PF_R (Read)",
        5 => "PF_R + PF_X (Read, execute)",
        6 => "PF_R + PF_W (Read, write)",
        7 => "PF_R + PF_W + PF_X (Read, write, execute)",
        0xf0000000 => "Unspecified",
        _ => "unknow"
    }
}





pub fn read_elf_headers(filepath: &str) -> Result<(), io::Error> {
    let mut file = File::open(filepath)?;

    let mut header_data = [0u8; 64];
    file.read_exact(&mut header_data)?;

    if &header_data[0..4] != b"\x7FELF" {
        return Err(io::Error::new(io::ErrorKind::Other, "invalid elf file"));
    }
    let machine = u16::from_le_bytes([header_data[18], header_data[19]]);
    if machine == 3 {
        Ok(x32::header_elf::header_elf(header_data, &mut file)?)
    }else if machine == 62 {
        Ok(x64::header_elf::pars_header(header_data, &mut file)?)
    }else {
        return Err(io::Error::new(io::ErrorKind::Other, "the architcture is not supported (".to_owned() + get_machine(machine) + ")"))
    }
}


