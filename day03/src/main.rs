use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Diagnostics {
    bits: [u32; 12],
    len: u32,
}

impl Diagnostics {
    fn new() -> Self {
        Self { bits: [0; 12], len: 0 }
    }

    fn read(&mut self, path: &str) -> io::Result<()> {
        let file = File::open(path)?;
        let buffer = BufReader::new(file);
        for line in buffer.lines() {
            for (i, b) in line?.chars().enumerate() {
                if b == '1' {
                    self.bits[i] += 1;
                }
            }
            self.len += 1;
        }
        Ok(())
    }

    fn gamma(&self) -> u32 {
        let cut = self.len / 2;
        let mut gamma = 0;
        for (i, bit) in self.bits.into_iter().enumerate() {
            if bit > cut {
                gamma += 1 << (11 - i);
            }
        }
        gamma
    }

    fn epsilon(&self) -> u32 {
        !self.gamma() & ((1 << 12) - 1)
    }
}

fn main() -> io::Result<()> {
    let mut log = Diagnostics::new();
    log.read("input")?;
    println!("{}", log.gamma() * log.epsilon());
    Ok(())
}
