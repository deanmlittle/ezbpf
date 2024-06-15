use std::io::Cursor;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    cursor::ELFCursor, elf_header::ELFHeader, errors::EZBpfError, program_header::ProgramHeader,
    section_header::SectionHeader, section_header_entry::SectionHeaderEntry,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    elf_header: ELFHeader,
    program_headers: Vec<ProgramHeader>,
    section_headers: Vec<SectionHeader>,
    section_header_entries: Vec<SectionHeaderEntry>,
}

impl Program {
    pub fn from_bytes(b: &[u8]) -> Result<Self, EZBpfError> {
        let mut c = Cursor::new(b);
        let elf_header = c.read_elf_header()?;

        c.set_position(elf_header.e_phoff);
        let program_headers = (0..elf_header.e_phnum)
            .map(|_| c.read_program_header())
            .collect::<Result<Vec<_>, _>>()?;
        c.set_position(elf_header.e_shoff);
        let section_headers = (0..elf_header.e_shnum)
            .map(|_| c.read_section_header())
            .collect::<Result<Vec<_>, _>>()?;

        let shstrndx = &section_headers[elf_header.e_shstrndx as usize];
        let shstrndx_value = b
            [shstrndx.sh_offset as usize..shstrndx.sh_offset as usize + shstrndx.sh_size as usize]
            .to_vec();

        let mut prev_section_header_offset = shstrndx_value.len();
        let mut section_header_entries = section_headers
            .clone()
            .iter()
            .rev()
            .map(|s| {
                let label = String::from_utf8(
                    shstrndx_value[s.sh_name as usize..prev_section_header_offset].to_vec(),
                )
                .unwrap();
                let data =
                    b[s.sh_offset as usize..s.sh_offset as usize + s.sh_size as usize].to_vec();
                prev_section_header_offset = s.sh_name as usize;

                SectionHeaderEntry {
                    label,
                    offset: s.sh_offset as usize,
                    data,
                }
            })
            .collect::<Vec<SectionHeaderEntry>>();
        section_header_entries.reverse();

        Ok(Self {
            elf_header,
            program_headers,
            section_headers,
            section_header_entries,
        })
    }
}
