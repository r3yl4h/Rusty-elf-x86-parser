use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};
use std::mem::size_of;
use crate::header_elf::{flag_peh_desc, type_head};
use crate::x32::section_elf;

pub struct ElfHeader32 {
    pub(crate) ident: [u8; 16],
    pub(crate) etype: u16,
    pub(crate) machine: u16,
    pub(crate) version: u32,
    pub(crate) entry: u32,
    pub(crate) ph_offset: u32,
    pub(crate) sh_offset: u32,
    pub(crate) flags: u32,
    pub(crate) eh_size: u16,
    pub(crate) ph_entry_size: u16,
    pub(crate) ph_num: u16,
    pub(crate) sh_entry_size: u16,
    pub(crate) sh_num: u16,
    pub(crate) shstrndx: u16,
}


#[repr(C)]
pub struct Peh32 {
    pub p_type: u32,
    pub p_offset: u32,
    pub p_vaddr: u32,
    pub p_paddr: u32,
    pub p_filesz: u32,
    pub p_memsz: u32,
    pub p_flags: u32,
    pub p_align: u32,
}



pub fn header_elf(header_data: [u8; 64], file: &mut File) -> Result<(), io::Error>{
    let elf_header = ElfHeader32 {
        ident: [
            header_data[0], header_data[1], header_data[2], header_data[3], header_data[4],
            header_data[5], header_data[6], header_data[7], header_data[8], header_data[9],
            header_data[10], header_data[11], header_data[12], header_data[13], header_data[14],
            header_data[15],
        ],
        etype: u16::from_le_bytes([header_data[16], header_data[17]]),
        machine: u16::from_le_bytes([header_data[18], header_data[19]]),
        version: u32::from_le_bytes([header_data[20], header_data[21], header_data[22], header_data[23]]),
        entry: u32::from_le_bytes([header_data[24], header_data[25], header_data[26], header_data[27]]),
        ph_offset: u32::from_le_bytes([header_data[28], header_data[29], header_data[30], header_data[31]]),
        sh_offset: u32::from_le_bytes([header_data[32], header_data[33], header_data[34], header_data[35]]),
        flags: u32::from_le_bytes([header_data[36], header_data[37], header_data[38], header_data[39]]),
        eh_size: u16::from_le_bytes([header_data[40], header_data[41]]),
        ph_entry_size: u16::from_le_bytes([header_data[42], header_data[43]]),
        ph_num: u16::from_le_bytes([header_data[44], header_data[45]]),
        sh_entry_size: u16::from_le_bytes([header_data[46], header_data[47]]),
        sh_num: u16::from_le_bytes([header_data[48], header_data[49]]),
        shstrndx: u16::from_le_bytes([header_data[50], header_data[51]]),
    };
    print_header32(&elf_header);
    pht32(file, elf_header.ph_num as usize)?;
    Ok(section_elf::pars_section(file, elf_header)?)
}




fn pht32(file: &mut File, count: usize) -> Result<(), io::Error> {
    let phdr_size = size_of::<Peh32>();
    file.seek(SeekFrom::Start(0x34))?;
    for i in 0..count {
        let mut buffer = vec![0; phdr_size];
        file.read_exact(&mut buffer)?;
        let phdr: Peh32 = unsafe { std::ptr::read(buffer.as_ptr() as *const _) };
        println!("--------------------------------");
        println!("   PHT: {}", i);
        println!("     type: {:#x} - {}", phdr.p_type, type_head(phdr.p_type));
        println!("     flags: {:#x} - {}", phdr.p_flags, flag_peh_desc(phdr.p_flags));
        println!("     Offset: {:#x}", phdr.p_offset);
        println!("     VA: {:#x} - PA: {:#x}", phdr.p_vaddr, phdr.p_paddr);
        println!("     File Size: {:#x} - Memory Size: {:#x}", phdr.p_filesz, phdr.p_memsz);
        println!("     Alignment: {:#x}", phdr.p_align);
        println!();
    }
    Ok(())
}



fn print_header32(elf_header: &ElfHeader32) {
    print!("     ELF Identification: [");
    for b in elf_header.ident{
        print!("{:#x}, ", b);
    }
    println!("]");
    println!("     Type: {} ({})", elf_header.etype, crate::header_elf::get_type_file(elf_header.etype));
    println!("     Machine: {}  ;{}", elf_header.machine, crate::header_elf::get_machine(elf_header.machine));
    println!("     Version: {}", elf_header.version);
    println!("     Entry point : {:#x}", elf_header.entry);
    println!("     program headers offset: {:#x}", elf_header.ph_offset);
    println!("     sh offset: {:#x}", elf_header.sh_offset);
    println!("     Flags: {}", elf_header.flags);
    println!("     Size of this header: {}", elf_header.eh_size);
    println!("     Size of pht: {}", elf_header.ph_entry_size);
    println!("     Number of pht: {}", elf_header.ph_num);
    println!("     Size of sh: {}", elf_header.sh_entry_size);
    println!("     Number of sh: {}", elf_header.sh_num);
    println!("     Sh string table index: {}", elf_header.shstrndx);
}