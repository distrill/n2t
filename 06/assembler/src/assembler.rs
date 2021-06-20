use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::{Result};

use crate::parser::{Line};

#[derive(Debug)]
pub struct Assembler {
  src: Vec<String>,
  bin: Vec<String>
}

impl Assembler {
    pub fn new(filename: &impl AsRef<Path>) -> Result<Assembler> {
	let file = File::open(filename)?;
	let buf = BufReader::new(file);
	let src = buf.lines()
	    .map(|l| l.expect("Could not parse line"))
	    .collect();

        let bin = Vec::new();
        Ok(Assembler{ src, bin })
    }

    pub fn process(&mut self) -> Result<()> {
        for l in &self.src {
            let line = Line::new(l)?;
            match line {
                Line::A{..} | Line::C{..} => {
                    self.bin.push(line.to_bin()?);
                },
                _ => {},
            };

        }
        Ok(())
    }

    pub fn write_bin(&self, binname: &String) -> Result<()> {
        let mut buf = "".to_string();
        for binline in &self.bin {
            buf.push_str(binline);
            buf.push('\n');
        }

        fs::write(binname, buf)?;
    
        Ok(())
    }
}
