/**
 * @author Mrack
 * @date 2022/12/13
 */

use core::slice;
use std::fs;
use std::path::Path;
use goblin::elf::{Elf, ProgramHeader};
use goblin::elf::program_header::PT_LOAD;
use crate::maps::MapItem;

pub struct Checker {
    m: MapItem,
    header: Option<ProgramHeader>,
    crc: u16,
}

pub fn checksum(msg: &[u8]) -> u16 {
    let mut crc: u16 = 0x0;
    for byte in msg.iter() {
        let mut x = ((crc >> 8) ^ (*byte as u16)) & 255;
        x ^= x >> 4;
        crc = (crc << 8) ^ (x << 12) ^ (x << 5) ^ x;
    }
    crc
}

impl Checker {
    pub fn new(m: MapItem) -> Checker {
        Checker {
            m,
            header: None,
            crc: 0,
        }
    }

    pub fn check(&mut self) -> Result<i64, &'static str> {
        let header = if self.header == None {
            let path = Path::new(self.m.pathname.as_str());
            let buffer = fs::read(path).unwrap().leak();
            let platform = Elf::parse(buffer);
            if platform.is_ok() {
                let platform = platform.unwrap();
                platform.program_headers.iter().for_each(|header| {
                    if header.p_type == PT_LOAD && header.is_executable() && header.is_read() {
                        if header.p_offset == self.m.offset {
                            if self.crc == 0 {
                                let len = header.p_memsz as usize;
                                let dest_data = &buffer.to_vec()[header.p_offset as usize..(header.p_offset as usize + len) as usize];
                                self.crc = checksum(dest_data);
                            }
                            self.header = Some(header.clone());
                        }
                    }
                });
            }
            self.header.clone()
        } else {
            self.header.clone()
        };
        if header.is_none() {
            return Err("No header found");
        }
        let len = header.unwrap().p_memsz as usize;
        let ptr_data = self.m.start as *const u8;
        let raw_data = unsafe { slice::from_raw_parts(ptr_data, len) };
        let crc1 = checksum(raw_data);
        if crc1 != self.crc {
            return Err("Elf is modified");
        }
        return Ok(0);
    }
}

#[cfg(test)]
mod tests {
    use crate::maps::ProcMaps;
    use crate::check::Checker;

    #[test]
    fn test() {
        let maps = ProcMaps::new().unwrap();
        for m in maps {
            if m.pathname.contains("libc") && m.perm == "r-xp" {
                if Checker::new(m).check().is_ok() {
                    println!("Elf is not modified");
                } else {
                    println!("Elf is modified");
                }
            }
        }
    }
}