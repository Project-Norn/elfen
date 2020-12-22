use section::SectionData;
use segment::ProgramHeader;

use crate::header::Header;
use crate::section::{Section, SectionHeader};
use crate::*;

pub struct Elf {
    pub header: Header,
    pub sections: Vec<Section>,
    pub segments: Vec<ProgramHeader>,
}

impl Default for Elf {
    fn default() -> Self {
        Self {
            header: Header::new(),
            sections: Vec::new(),
            segments: Vec::new(),
        }
    }
}

impl Elf {
    pub fn add_section(&mut self, name: &str, header: SectionHeader, data: SectionData) {
        self.sections.push(Section {
            name: name.to_string(),
            header,
            data,
        });
    }

    pub fn add_segment(&mut self, header: ProgramHeader) {
        self.segments.push(header);
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.header.write_to(&mut result);
        for segment in &self.segments {
            segment.write_to(&mut result);
        }
        for section in &self.sections {
            add_padding(&mut result, section.header.offset as usize);
            section.data.write_to(&mut result);
        }

        add_padding(&mut result, self.header.section_header_offset as usize);
        for section in &self.sections {
            section.header.write_to(&mut result);
        }
        result
    }
}

fn add_padding(v: &mut Vec<u8>, offset: usize) {
    if offset < v.len() {
        return;
    }
    let padding_len = offset - v.len();
    v.extend(&vec![0; padding_len as usize]);
}
