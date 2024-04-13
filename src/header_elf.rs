use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};
use std::mem::size_of;
use crate::section_elf;


pub struct ElfHeader {
    pub(crate) ident: [u8; 16],
    pub(crate) etype: u16,
    pub(crate) machine: u16,
    pub(crate) version: u32,
    pub(crate) entry: u64,
    pub(crate) ph_offset: u64,
    pub(crate) sh_offset: u64,
    pub(crate) flags: u32,
    pub(crate) eh_size: u16,
    pub(crate) ph_entry_size: u16,
    pub(crate) ph_num: u16,
    pub(crate) sh_entry_size: u16,
    pub(crate) sh_num: u16,
    pub(crate) shstrndx: u16,
}

fn get_type_file(types: u16) -> &'static str {
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

fn get_machine(machine: u16) -> &'static str {
    match machine {
        1 => "AT&T WE 32100",
        2 => "SPARC",
        3 => "Intel 80386",
        4 => "Motorola 68000",
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
        19 => "Intel 80960",
        20 => "PowerPC",
        40 => "ARM",
        50 => "Intel IA64",
        62 => "x86-64",
        243 => "RISC-V",
        _ => "unknown",
    }
}


fn type_head(p_type: u32) -> &'static str {
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

fn flag_peh_desc(flags: u32) -> &'static str{
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

fn print_header(elf_header: &ElfHeader) {
    print!("     ELF Identification: [");
    for b in elf_header.ident{
        print!("{:#x}, ", b);
    }
    println!("]");
    println!("     Type: {} ({})", elf_header.etype, get_type_file(elf_header.etype));
    println!("     Machine: {}  ;{}", elf_header.machine, get_machine(elf_header.machine));
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


#[repr(C)]
#[derive(Debug)]
pub struct Peh {
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}




pub fn read_elf_headers(filepath: &str) -> Result<(), io::Error> {
    let mut file = File::open(filepath)?;

    let mut header_data = [0u8; 64];
    file.read_exact(&mut header_data)?;

    if &header_data[0..4] != b"\x7FELF" {
        return Err(io::Error::new(io::ErrorKind::Other, "invalid elf file"));
    }

    let elf_header = ElfHeader {
        ident: [
            header_data[0], header_data[1], header_data[2], header_data[3], header_data[4],
            header_data[5], header_data[6], header_data[7], header_data[8], header_data[9],
            header_data[10], header_data[11], header_data[12], header_data[13], header_data[14],
            header_data[15],
        ],
        etype: u16::from_le_bytes([header_data[16], header_data[17]]),
        machine: u16::from_le_bytes([header_data[18], header_data[19]]),
        version: u32::from_le_bytes([header_data[20], header_data[21], header_data[22], header_data[23]]),
        entry: u64::from_le_bytes([header_data[24], header_data[25], header_data[26], header_data[27], header_data[28],
            header_data[29], header_data[30], header_data[31]
        ]),
        ph_offset: u64::from_le_bytes([
            header_data[32], header_data[33], header_data[34], header_data[35], header_data[36],
            header_data[37], header_data[38], header_data[39]
        ]),
        sh_offset: u64::from_le_bytes([header_data[40], header_data[41], header_data[42], header_data[43], header_data[44],
            header_data[45], header_data[46], header_data[47]
        ]),
        flags: u32::from_le_bytes([header_data[48], header_data[49], header_data[50], header_data[51]]),
        eh_size: u16::from_le_bytes([header_data[52], header_data[53]]),
        ph_entry_size: u16::from_le_bytes([header_data[54], header_data[55]]),
        ph_num: u16::from_le_bytes([header_data[56], header_data[57]]),
        sh_entry_size: u16::from_le_bytes([header_data[58], header_data[59]]),
        sh_num: u16::from_le_bytes([header_data[60], header_data[61]]),
        shstrndx: u16::from_le_bytes([header_data[62], header_data[63]]),
    };
    print_header(&elf_header);
    pht(&mut file, elf_header.ph_num as usize)?;
    section_elf::pars_section(&mut file, elf_header)?;
    Ok(())
}

fn pht(file: &mut File, count: usize) -> Result<(), io::Error>{
    let phdr_size = size_of::<Peh>();
    file.seek(SeekFrom::Start(0x40))?;
    for i in 0..count {
        let mut buffer = vec![0; phdr_size];
        file.read_exact(&mut buffer)?;
        let phdr: Peh = unsafe { std::ptr::read(buffer.as_ptr() as *const _) };
        println!("--------------------------------");
        println!("   PHT: {i}");
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