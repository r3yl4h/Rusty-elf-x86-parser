# Rusty-elf64-parser
it's a small program written in rust with only the standard library to analyze the headers of an elf64 file

## output
```
 C:\code\rust\rusty_Elf_parser\target\debug> C:\code\rust\rusty_Elf_parser\target\debug\rusty_Elf_parser.exe "C:\test\main" 
     ELF Identification: [0x7f, 0x45, 0x4c, 0x46, 0x2, 0x1, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, ]
     Type: 3 (Shared object file)
     Machine: 62  ;x86-64
     Version: 1
     Entry point : 0x8a40
     program headers offset: 0x40
     sh offset: 0xc4c1d8
     Flags: 0
     Size of this header: 64
     Size of pht: 56
     Number of pht: 14
     Size of sh: 64
     Number of sh: 43
     Sh string table index: 42
```
```
--------------------------------
   PHT: 0
     type: 0x6 - PT_PHDR
     flags: 0x4 - PF_R (Read)
     Offset: 0x40
     VA: 0x40 - PA: 0x40
     File Size: 0x310 - Memory Size: 0x310
     Alignment: 0x8

--------------------------------
   PHT: 1
     type: 0x3 - PT_INTERP
     flags: 0x4 - PF_R (Read)
     Offset: 0x350
     VA: 0x350 - PA: 0x350
     File Size: 0x1c - Memory Size: 0x1c
     Alignment: 0x1

--------------------------------
   PHT: 2
     type: 0x1 - PT_LOAD
     flags: 0x4 - PF_R (Read)
     Offset: 0x0
     VA: 0x0 - PA: 0x0
     File Size: 0x5548 - Memory Size: 0x5548
     Alignment: 0x1000

--------------------------------
   PHT: 3
     type: 0x1 - PT_LOAD
     flags: 0x5 - PF_R + PF_X (Read, execute)
     Offset: 0x6000
     VA: 0x6000 - PA: 0x6000
     File Size: 0x38e3d - Memory Size: 0x38e3d
     Alignment: 0x1000

--------------------------------
   PHT: 4
     type: 0x1 - PT_LOAD
     flags: 0x4 - PF_R (Read)
     Offset: 0x3f000
     VA: 0x3f000 - PA: 0x3f000
     File Size: 0xe6c0 - Memory Size: 0xe6c0
     Alignment: 0x1000

--------------------------------
   PHT: 5
     type: 0x1 - PT_LOAD
     flags: 0x6 - PF_R + PF_W (Read, write)
     Offset: 0x4e0d0
     VA: 0x4e0d0 - PA: 0x4e0d0
     File Size: 0x2f60 - Memory Size: 0x3068
     Alignment: 0x1000

--------------------------------
   PHT: 6
     type: 0x2 - PT_DYNAMIC
     flags: 0x6 - PF_R + PF_W (Read, write)
     Offset: 0x50738
     VA: 0x50738 - PA: 0x50738
     File Size: 0x210 - Memory Size: 0x210
     Alignment: 0x8

--------------------------------
   PHT: 7
     type: 0x4 - PT_NOTE
     flags: 0x4 - PF_R (Read)
     Offset: 0x370
     VA: 0x370 - PA: 0x370
     File Size: 0x20 - Memory Size: 0x20
     Alignment: 0x8

--------------------------------
   PHT: 8
     type: 0x4 - PT_NOTE
     flags: 0x4 - PF_R (Read)
     Offset: 0x390
     VA: 0x390 - PA: 0x390
     File Size: 0x44 - Memory Size: 0x44
     Alignment: 0x4

--------------------------------
   PHT: 9
     type: 0x7 - PT_TLS
     flags: 0x4 - PF_R (Read)
     Offset: 0x4e0d0
     VA: 0x4e0d0 - PA: 0x4e0d0
     File Size: 0x28 - Memory Size: 0x4a
     Alignment: 0x8

--------------------------------
   PHT: 10
     type: 0x6474e553 - UNKNOW
     flags: 0x4 - PF_R (Read)
     Offset: 0x370
     VA: 0x370 - PA: 0x370
     File Size: 0x20 - Memory Size: 0x20
     Alignment: 0x8

--------------------------------
   PHT: 11
     type: 0x6474e550 - EH_FRAME
     flags: 0x4 - PF_R (Read)
     Offset: 0x43c34
     VA: 0x43c34 - PA: 0x43c34
     File Size: 0x150c - Memory Size: 0x150c
     Alignment: 0x4

--------------------------------
   PHT: 12
     type: 0x6474e551 - PT_STACK
     flags: 0x6 - PF_R + PF_W (Read, write)
     Offset: 0x0
     VA: 0x0 - PA: 0x0
     File Size: 0x0 - Memory Size: 0x0
     Alignment: 0x10

--------------------------------
   PHT: 13
     type: 0x6474e552 - RO-AFTER
     flags: 0x4 - PF_R (Read)
     Offset: 0x4e0d0
     VA: 0x4e0d0 - PA: 0x4e0d0
     File Size: 0x2f30 - Memory Size: 0x2f30
     Alignment: 0x1

```
```
======================================================================================================================================

-----------------------------------
 SEH: 0
    Name:
    offset: 0x0 - load-addr: 0x0
    size: 0x0
    algn: 0
    flag: 0 -
    types: 0x0 - SHT_NULL

-----------------------------------
 SEH: 1
    Name: .interp
    offset: 0x350 - load-addr: 0x350
    size: 0x1c
    algn: 1
    flag: 2 - SHF_ALLOC (readable)
    types: 0x1 - SHT_PROGBITS

-----------------------------------
 SEH: 2
    Name: .note.gnu.property
    offset: 0x370 - load-addr: 0x370
    size: 0x20
    algn: 8
    flag: 2 - SHF_ALLOC (readable)
    types: 0x7 - SHT_NOTE

-----------------------------------
 SEH: 3
    Name: .note.gnu.build-id
    offset: 0x390 - load-addr: 0x390
    size: 0x24
    algn: 4
    flag: 2 - SHF_ALLOC (readable)
    types: 0x7 - SHT_NOTE

-----------------------------------
 SEH: 4
    Name: .note.ABI-tag
    offset: 0x3b4 - load-addr: 0x3b4
    size: 0x20
    algn: 4
    flag: 2 - SHF_ALLOC (readable)
    types: 0x7 - SHT_NOTE

-----------------------------------
 SEH: 5
    Name: .gnu.hash
    offset: 0x3d8 - load-addr: 0x3d8
    size: 0x24
    algn: 8
    flag: 2 - SHF_ALLOC (readable)
    types: 0x6ffffff6 - unknown

-----------------------------------
 SEH: 6
    Name: .dynsym
    offset: 0x400 - load-addr: 0x400
    size: 0x618
    algn: 8
    flag: 2 - SHF_ALLOC (readable)
    types: 0xb - SHT_DYNSYM (initialized data)

-----------------------------------
 SEH: 7
    Name: .dynstr
    offset: 0xa18 - load-addr: 0xa18
    size: 0x3ec
    algn: 1
    flag: 2 - SHF_ALLOC (readable)
    types: 0x3 - SHT_STRTAB (initialized data)

-----------------------------------
 SEH: 8
    Name: .gnu.version
    offset: 0xe04 - load-addr: 0xe04
    size: 0x82
    algn: 2
    flag: 2 - SHF_ALLOC (readable)
    types: 0x6fffffff - unknown

-----------------------------------
 SEH: 9
    Name: .gnu.version_r
    offset: 0xe88 - load-addr: 0xe88
    size: 0x100
    algn: 8
    flag: 2 - SHF_ALLOC (readable)
    types: 0x6ffffffe - unknown

-----------------------------------
 SEH: 10
    Name: .rela.dyn
    offset: 0xf88 - load-addr: 0xf88
    size: 0x4590
    algn: 8
    flag: 2 - SHF_ALLOC (readable)
    types: 0x4 - SHT_RELA

-----------------------------------
 SEH: 11
    Name: .rela.plt
    offset: 0x5518 - load-addr: 0x5518
    size: 0x30
    algn: 8
    flag: 66 -
    types: 0x4 - SHT_RELA

-----------------------------------
 SEH: 12
    Name: .init
    offset: 0x6000 - load-addr: 0x6000
    size: 0x17
    algn: 4
    flag: 6 -
    types: 0x1 - SHT_PROGBITS

-----------------------------------
 SEH: 13
    Name: .plt
    offset: 0x6020 - load-addr: 0x6020
    size: 0x30
    algn: 16
    flag: 6 -
    types: 0x1 - SHT_PROGBITS

-----------------------------------
 SEH: 14
    Name: .plt.got
    offset: 0x6050 - load-addr: 0x6050
    size: 0x8
    algn: 8
    flag: 6 -
    types: 0x1 - SHT_PROGBITS

-----------------------------------
 SEH: 15
    Name: .text
    offset: 0x6060 - load-addr: 0x6060
    size: 0x38dd4
    algn: 16
    flag: 6 -
    types: 0x1 - SHT_PROGBITS

-----------------------------------
 SEH: 16
    Name: .fini
    offset: 0x3ee34 - load-addr: 0x3ee34
    size: 0x9
    algn: 4
    flag: 6 -
    types: 0x1 - SHT_PROGBITS

-----------------------------------
 SEH: 17
    Name: .rodata
    offset: 0x3f000 - load-addr: 0x3f000
    size: 0x4c34
    algn: 16
    flag: 2 - SHF_ALLOC (readable)
    types: 0x1 - SHT_PROGBITS

-----------------------------------
 SEH: 18
    Name: .eh_frame_hdr
    offset: 0x43c34 - load-addr: 0x43c34
    size: 0x150c
    algn: 4
    flag: 2 - SHF_ALLOC (readable)
    types: 0x1 - SHT_PROGBITS

-----------------------------------
 SEH: 19
    Name: .eh_frame
    offset: 0x45140 - load-addr: 0x45140
    size: 0x7660
    algn: 8
    flag: 2 - SHF_ALLOC (readable)
    types: 0x1 - SHT_PROGBITS

-----------------------------------
 SEH: 20
    Name: .gcc_except_table
    offset: 0x4c7a0 - load-addr: 0x4c7a0
    size: 0xf20
    algn: 4
    flag: 2 - SHF_ALLOC (readable)
    types: 0x1 - SHT_PROGBITS

-----------------------------------
 SEH: 21
    Name: .tdata
    offset: 0x4e0d0 - load-addr: 0x4e0d0
    size: 0x28
    algn: 8
    flag: 1027 -
    types: 0x1 - SHT_PROGBITS

-----------------------------------
 SEH: 22
    Name: .tbss
    offset: 0x4e0f8 - load-addr: 0x4e0f8
    size: 0x22
    algn: 8
    flag: 1027 -
    types: 0x8 - SHT_NOBITS (uninitialized data)

-----------------------------------
 SEH: 23
    Name: .init_array
    offset: 0x4e0f8 - load-addr: 0x4e0f8
    size: 0x10
    algn: 8
    flag: 3 -
    types: 0xe - unknown

-----------------------------------
 SEH: 24
    Name: .fini_array
    offset: 0x4e108 - load-addr: 0x4e108
    size: 0x8
    algn: 8
    flag: 3 -
    types: 0xf - unknown

-----------------------------------
 SEH: 25
    Name: .data.rel.ro
    offset: 0x4e110 - load-addr: 0x4e110
    size: 0x2628
    algn: 8
    flag: 3 -
    types: 0x1 - SHT_PROGBITS

-----------------------------------
 SEH: 26
    Name: .dynamic
    offset: 0x50738 - load-addr: 0x50738
    size: 0x210
    algn: 8
    flag: 3 -
    types: 0x6 - SHT_DYNAMIC (initialized data)

-----------------------------------
 SEH: 27
    Name: .got
    offset: 0x50948 - load-addr: 0x50948
    size: 0x6b8
    algn: 8
    flag: 3 -
    types: 0x1 - SHT_PROGBITS

-----------------------------------
 SEH: 28
    Name: .data
    offset: 0x51000 - load-addr: 0x51000
    size: 0x30
    algn: 8
    flag: 3 -
    types: 0x1 - SHT_PROGBITS

-----------------------------------
 SEH: 29
    Name: .bss
    offset: 0x51030 - load-addr: 0x51030
    size: 0x108
    algn: 8
    flag: 3 -
    types: 0x8 - SHT_NOBITS (uninitialized data)

-----------------------------------
 SEH: 30
    Name: .comment
    offset: 0x51030 - load-addr: 0x0
    size: 0x1f
    algn: 1
    flag: 48 -
    types: 0x1 - SHT_PROGBITS

-----------------------------------
 SEH: 31
    Name: .debug_aranges
    offset: 0x5104f - load-addr: 0x0
    size: 0x10e20
    algn: 1
    flag: 0 -
    types: 0x1 - SHT_PROGBITS

-----------------------------------
 SEH: 32
    Name: .debug_pubnames
    offset: 0x61e6f - load-addr: 0x0
    size: 0x10c58c
    algn: 1
    flag: 0 -
    types: 0x1 - SHT_PROGBITS

-----------------------------------
 SEH: 33
    Name: .debug_info
    offset: 0x16e3fb - load-addr: 0x0
    size: 0x384733
    algn: 1
    flag: 0 -
    types: 0x1 - SHT_PROGBITS

-----------------------------------
 SEH: 34
    Name: .debug_abbrev
    offset: 0x4f2b2e - load-addr: 0x0
    size: 0xd248
    algn: 1
    flag: 0 -
    types: 0x1 - SHT_PROGBITS

-----------------------------------
 SEH: 35
    Name: .debug_line
    offset: 0x4ffd76 - load-addr: 0x0
    size: 0x8bec6
    algn: 1
    flag: 0 -
    types: 0x1 - SHT_PROGBITS

-----------------------------------
 SEH: 36
    Name: .debug_str
    offset: 0x58bc3c - load-addr: 0x0
    size: 0x1bdc33
    algn: 1
    flag: 48 -
    types: 0x1 - SHT_PROGBITS

-----------------------------------
 SEH: 37
    Name: .debug_loc
    offset: 0x74986f - load-addr: 0x0
    size: 0x2d9733
    algn: 1
    flag: 0 -
    types: 0x1 - SHT_PROGBITS

-----------------------------------
 SEH: 38
    Name: .debug_pubtypes
    offset: 0xa22fa2 - load-addr: 0x0
    size: 0x1197bb
    algn: 1
    flag: 0 -
    types: 0x1 - SHT_PROGBITS

-----------------------------------
 SEH: 39
    Name: .debug_ranges
    offset: 0xb3c75d - load-addr: 0x0
    size: 0xf41e0
    algn: 1
    flag: 0 -
    types: 0x1 - SHT_PROGBITS

-----------------------------------
 SEH: 40
    Name: .symtab
    offset: 0xc30940 - load-addr: 0x0
    size: 0x7410
    algn: 8
    flag: 0 -
    types: 0x2 - SHT_SYMTAB (initialized data)

-----------------------------------
 SEH: 41
    Name: .strtab
    offset: 0xc37d50 - load-addr: 0x0
    size: 0x142d2
    algn: 1
    flag: 0 -
    types: 0x3 - SHT_STRTAB (initialized data)

-----------------------------------
 SEH: 42
    Name: .shstrtab
    offset: 0xc4c022 - load-addr: 0x0
    size: 0x1b6
    algn: 1
    flag: 0 -
    types: 0x3 - SHT_STRTAB (initialized data)

```
```
dynsym table offset: 0x400 - size: 0x618
string table offset: 0xa18 - size: 0x3ec

-----------------------------------
 SYH: 0
    offset of sym: 0x400
    address offset name = 0x0
    st_infos = 0x0
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 1
    offset of sym: 0x418
    address offset name = 0x269
    name: ('mprotect') - offset: 0xc81
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 2
    offset of sym: 0x430
    address offset name = 0xb0
    name: ('_Unwind_GetRegionStart') - offset: 0xac8
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 3
    offset of sym: 0x448
    address offset name = 0x2a8
    name: ('memset') - offset: 0xcc0
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 4
    offset of sym: 0x460
    address offset name = 0x112
    name: ('_Unwind_SetGR') - offset: 0xb2a
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 5
    offset of sym: 0x478
    address offset name = 0x1a0
    name: ('mmap64') - offset: 0xbb8
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 6
    offset of sym: 0x490
    address offset name = 0x310
    name: ('posix_memalign') - offset: 0xd28
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 7
    offset of sym: 0x4a8
    address offset name = 0x2c3
    name: ('close') - offset: 0xcdb
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 8
    offset of sym: 0x4c0
    address offset name = 0x99
    name: ('_Unwind_GetDataRelBase') - offset: 0xab1
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 9
    offset of sym: 0x4d8
    address offset name = 0x31f
    name: ('abort') - offset: 0xd37
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 10
    offset of sym: 0x4f0
    address offset name = 0x2af
    name: ('pthread_setspecific') - offset: 0xcc7
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 11
    offset of sym: 0x508
    address offset name = 0x1d
    name: ('__gmon_start__') - offset: 0xa35
    st_infos = 0x20
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 12
    offset of sym: 0x520
    address offset name = 0x227
    name: ('malloc') - offset: 0xc3f
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 13
    offset of sym: 0x538
    address offset name = 0x2d0
    name: ('pthread_getattr_np') - offset: 0xce8
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 14
    offset of sym: 0x550
    address offset name = 0xe3
    name: ('_Unwind_DeleteException') - offset: 0xafb
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 15
    offset of sym: 0x568
    address offset name = 0x151
    name: ('sysconf') - offset: 0xb69
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 16
    offset of sym: 0x580
    address offset name = 0x202
    name: ('pthread_attr_destroy') - offset: 0xc1a
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 17
    offset of sym: 0x598
    address offset name = 0x1
    name: ('_ITM_deregisterTMCloneTable') - offset: 0xa19
    st_infos = 0x20
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 18
    offset of sym: 0x5b0
    address offset name = 0x67
    name: ('_Unwind_GetLanguageSpecificData') - offset: 0xa7f
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 19
    offset of sym: 0x5c8
    address offset name = 0x159
    name: ('free') - offset: 0xb71
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 20
    offset of sym: 0x5e0
    address offset name = 0x1d6
    name: ('strlen') - offset: 0xbee
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 21
    offset of sym: 0x5f8
    address offset name = 0x2c
    name: ('_ITM_registerTMCloneTable') - offset: 0xa44
    st_infos = 0x20
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 22
    offset of sym: 0x610
    address offset name = 0x273
    name: ('stat64') - offset: 0xc8b
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 23
    offset of sym: 0x628
    address offset name = 0x1a7
    name: ('__cxa_thread_atexit_impl') - offset: 0xbbf
    st_infos = 0x22
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 24
    offset of sym: 0x640
    address offset name = 0xfb
    name: ('_Unwind_RaiseException') - offset: 0xb13
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 25
    offset of sym: 0x658
    address offset name = 0x181
    name: ('realpath') - offset: 0xb99
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 26
    offset of sym: 0x670
    address offset name = 0x332
    name: ('__tls_get_addr') - offset: 0xd4a
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 27
    offset of sym: 0x688
    address offset name = 0x240
    name: ('pthread_key_create') - offset: 0xc58
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 28
    offset of sym: 0x6a0
    address offset name = 0x308
    name: ('syscall') - offset: 0xd20
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 29
    offset of sym: 0x6b8
    address offset name = 0xd5
    name: ('_Unwind_GetIP') - offset: 0xaed
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 30
    offset of sym: 0x6d0
    address offset name = 0x55
    name: ('_Unwind_Backtrace') - offset: 0xa6d
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 31
    offset of sym: 0x6e8
    address offset name = 0x164
    name: ('pthread_self') - offset: 0xb7c
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 32
    offset of sym: 0x700
    address offset name = 0x1c0
    name: ('poll') - offset: 0xbd8
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 33
    offset of sym: 0x718
    address offset name = 0x18a
    name: ('open64') - offset: 0xba2
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 34
    offset of sym: 0x730
    address offset name = 0x171
    name: ('sigaction') - offset: 0xb89
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 35
    offset of sym: 0x748
    address offset name = 0x272
    name: ('fstat64') - offset: 0xc8a
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 36
    offset of sym: 0x760
    address offset name = 0x289
    name: ('bcmp') - offset: 0xca1
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 37
    offset of sym: 0x778
    address offset name = 0x253
    name: ('pthread_attr_getstack') - offset: 0xc6b
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 38
    offset of sym: 0x790
    address offset name = 0x32a
    name: ('memrchr') - offset: 0xd42
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 39
    offset of sym: 0x7a8
    address offset name = 0x1f1
    name: ('readlink') - offset: 0xc09
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 40
    offset of sym: 0x7c0
    address offset name = 0x2c9
    name: ('signal') - offset: 0xce1
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 41
    offset of sym: 0x7d8
    address offset name = 0x198
    name: ('memmove') - offset: 0xbb0
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 42
    offset of sym: 0x7f0
    address offset name = 0x28e
    name: ('getenv') - offset: 0xca6
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 43
    offset of sym: 0x808
    address offset name = 0x87
    name: ('_Unwind_GetIPInfo') - offset: 0xa9f
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 44
    offset of sym: 0x820
    address offset name = 0x217
    name: ('dl_iterate_phdr') - offset: 0xc2f
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 45
    offset of sym: 0x838
    address offset name = 0x2f1
    name: ('__errno_location') - offset: 0xd09
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 46
    offset of sym: 0x850
    address offset name = 0x13e
    name: ('pthread_key_delete') - offset: 0xb56
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 47
    offset of sym: 0x868
    address offset name = 0x2ea
    name: ('getcwd') - offset: 0xd02
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 48
    offset of sym: 0x880
    address offset name = 0x1dd
    name: ('pthread_getspecific') - offset: 0xbf5
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 49
    offset of sym: 0x898
    address offset name = 0x295
    name: ('calloc') - offset: 0xcad
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 50
    offset of sym: 0x8b0
    address offset name = 0x191
    name: ('munmap') - offset: 0xba9
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 51
    offset of sym: 0x8c8
    address offset name = 0x1c5
    name: ('__xpg_strerror_r') - offset: 0xbdd
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 52
    offset of sym: 0x8e0
    address offset name = 0x137
    name: ('writev') - offset: 0xb4f
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 53
    offset of sym: 0x8f8
    address offset name = 0x120
    name: ('_Unwind_GetTextRelBase') - offset: 0xb38
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 54
    offset of sym: 0x910
    address offset name = 0x1fa
    name: ('realloc') - offset: 0xc12
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 55
    offset of sym: 0x928
    address offset name = 0x22e
    name: ('__libc_start_main') - offset: 0xc46
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 56
    offset of sym: 0x940
    address offset name = 0x302
    name: ('write') - offset: 0xd1a
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 57
    offset of sym: 0x958
    address offset name = 0x15e
    name: ('statx') - offset: 0xb76
    st_infos = 0x22
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 58
    offset of sym: 0x970
    address offset name = 0x46
    name: ('_Unwind_Resume') - offset: 0xa5e
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 59
    offset of sym: 0x988
    address offset name = 0x29c
    name: ('sigaltstack') - offset: 0xcb4
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 60
    offset of sym: 0x9a0
    address offset name = 0x2e3
    name: ('memcpy') - offset: 0xcfb
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 61
    offset of sym: 0x9b8
    address offset name = 0x17b
    name: ('fcntl') - offset: 0xb93
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 62
    offset of sym: 0x9d0
    address offset name = 0x325
    name: ('mmap') - offset: 0xd3d
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 63
    offset of sym: 0x9e8
    address offset name = 0xc7
    name: ('_Unwind_SetIP') - offset: 0xadf
    st_infos = 0x12
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0

-----------------------------------
 SYH: 64
    offset of sym: 0xa00
    address offset name = 0x27a
    name: ('__cxa_finalize') - offset: 0xc92
    st_infos = 0x22
    st_other = 0x0 - st_shndx = 0
    st_value = 0x0 - st_size = 0
```
