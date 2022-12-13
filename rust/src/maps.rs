/**
 * @author Mrack
 * @date 2022/12/13
 */

use std::fs::File;
use std::io::Read;
use anyhow::Result;

#[derive(Debug)]
pub struct MapItem {
    pub start: u64,
    pub end: u64,
    pub perm: String,
    pub offset: u64,
    pub dev: String,
    pub inode: u64,
    pub pathname: String,
}

pub struct ProcMaps {
    lines: Vec<MapItem>,
}

impl Iterator for ProcMaps {
    type Item = MapItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.lines.is_empty() {
            None
        } else {
            Some(self.lines.remove(0))
        }
    }
}

impl ProcMaps {
    pub fn new() -> Result<Self, &'static str> {
        let mut file = File::open("/proc/self/maps").map_err(|_| "Cannot open file")?;
        let mut content = String::new();
        file.read_to_string(&mut content).map_err(|_| "Cannot read file")?;
        let mut lines = Vec::new();
        for line in content.split("\n") {
            let mut iter = line.split_whitespace();
            let start_end = iter.next();
            if start_end.is_none() {
                continue;
            }
            let start_end = start_end.unwrap();
            let perm = iter.next().unwrap();
            let offset = iter.next().unwrap();
            let dev = iter.next().unwrap();
            let inode = iter.next().unwrap();
            let pathname = iter.next();
            if pathname.is_none() {
                continue;
            }
            let pathname = pathname.unwrap();
            let mut start_end_iter = start_end.split('-');
            let start = start_end_iter.next().unwrap();
            let end = start_end_iter.next().unwrap();
            lines.push(MapItem {
                start: u64::from_str_radix(start, 16).unwrap(),
                end: u64::from_str_radix(end, 16).unwrap(),
                perm: perm.to_string(),
                offset: u64::from_str_radix(offset, 16).unwrap(),
                dev: dev.to_string(),
                inode: u64::from_str_radix(inode, 10).unwrap(),
                pathname: pathname.clone().to_string(),
            });
        }

        Ok(Self {
            lines
        })
    }
}

#[allow(dead_code)]
fn get_maps(lib: &str) -> Option<MapItem> {
    let maps = ProcMaps::new().unwrap();
    for x in maps.lines {
        if x.pathname.contains(lib) {
            return Some(x);
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_maps_test() {
        println!("{:?}", get_maps("libc"));
        assert!(get_maps("libc").is_some());
    }

    #[test]
    fn proc_maps_test() {
        let maps = ProcMaps::new().unwrap();
        for x in maps {
            println!("{:?}", x);
        }
    }
}