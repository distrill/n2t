use std::env;

use anyhow::{anyhow, Result};

mod parser;
mod assembler;

use assembler::Assembler;

#[derive(Debug)]
struct Config {
    srcname: String,
    binname: String,
}

impl Config {
    fn parse(args: Vec<String>) -> Result<Config> {
        if args.len() < 2 {
            return Err(anyhow!("not enough arguments"));
        }
        let srcname = args[1].clone();
        if !srcname.ends_with(".asm") {
            return Err(anyhow!("file must be an asm file. (provided: {})", srcname));
        }

        let binname = srcname.replace(".asm", ".hack");
        Ok(Config { srcname, binname })
    }
}



fn main() -> Result<()>  {
    let config = Config::parse(env::args().collect())?;
    let mut assember = Assembler::new(&config.srcname)?;

    assember.process()?;
    assember.write_bin(&config.binname)?;
    
    Ok(())
}
