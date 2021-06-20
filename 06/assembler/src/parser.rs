use anyhow::{anyhow, Result};
use stringsort::insertsort;

/// we implement a double parse here, from string to token, and then from token
/// to binary string. with an assember as simple as this one, we could have gone
/// straight from string to bin string, but this way we retain semantics for 
/// each parsed token
///
/// among other things, this allows us to print the tokens with human readablj
/// names and descriptions
trait Token<T> {
    fn new (raw: Option<String>) -> Result<T>;
    fn to_bin(&self) -> String;
}

#[derive(Debug)]
pub enum Dest {
    Null,
    M,
    D,
    DM,
    A,
    AM,
    AD,
    ADM,
}

impl Token<Dest> for Dest {
    fn new(dest: Option<String>) -> Result<Dest> {
        use Dest::*;
        match dest {
            None => Ok(Null),
            Some(d) => {
                // alphabetize here to support any ordered combination
                // of AMD
                let alpha = insertsort(&d[..]);
                match &alpha[..] {
                    "M" => Ok(M),
                    "D" => Ok(D),
                    "DM" => Ok(DM),
                    "A" => Ok(A),
                    "AM" => Ok(AM),
                    "AD" => Ok(AD),
                    "ADM" => Ok(ADM),
                    _ => Err(anyhow!("unexpected dest token : {}", d)),
                }
            }
        }
    }
    fn to_bin(&self) -> String {
        use Dest::*;
        match &self {
            Null => "000".to_string(),
            M =>    "001".to_string(),
            D =>    "010".to_string(),
            DM =>   "011".to_string(),
            A =>    "100".to_string(),
            AM =>   "101".to_string(),
            AD =>   "110".to_string(),
            ADM =>  "111".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Comp {
    Zero,
    One,
    MinusOne,
    D,
    A,
    M,
    NotD,
    NotA,
    NotM,
    MinusD,
    MinusA,
    MinusM,
    DPlusOne,
    APlusOne,
    MPlusOne,
    DMinusOne,
    AMinusOne,
    MMinusOne,
    DPlusA,
    DPlusM,
    DMinusA,
    DMinusM,
    AMinusD,
    MMinusD,
    DAndA,
    DAndM,
    DOrA,
    DOrM,
}

impl Token<Comp> for Comp {
    fn new(comp: Option<String>) -> Result<Comp> {
        use Comp::*;
        match comp {
            Some(c) => {
                match &c.replace(" ", "")[..] {
                    "0" => Ok(Zero),
                    "1" => Ok(One),
                    "-1" => Ok(MinusOne),
                    "D" => Ok(D),
                    "A" => Ok(A),
                    "M" => Ok(M),
                    "!D" => Ok(NotD),
                    "!A" => Ok(NotA),
                    "!M" => Ok(NotM),
                    "-D" => Ok(MinusD),
                    "-A" => Ok(MinusA),
                    "-M" => Ok(MinusM),
                    "D+1" => Ok(DPlusOne),
                    "A+1" => Ok(APlusOne),
                    "M+1" => Ok(MPlusOne),
                    "D-1" => Ok(DMinusOne),
                    "A-1" => Ok(AMinusOne),
                    "M-1" => Ok(MMinusOne),
                    "D+A" => Ok(DPlusA),
                    "D+M" => Ok(DPlusM),
                    "D-A" => Ok(DMinusA),
                    "D-M" => Ok(DMinusM),
                    "A-D" => Ok(AMinusD),
                    "M-D" => Ok(MMinusD),
                    "D&A" => Ok(DAndA),
                    "D&M" => Ok(DAndM),
                    "D|A" => Ok(DOrA),
                    "D|M" => Ok(DOrM),
                    _ => Err(anyhow!("unexpected comp token: {}", c)),
                }
            },
            None => Err(anyhow!("Comp token is not optional")),
        }
    }
    fn to_bin(&self) -> String {
        use Comp::*;
        match &self {
            Zero =>         "0101010".to_string(),
            One =>          "0111111".to_string(),
            MinusOne =>     "0111010".to_string(),
            D =>            "0001100".to_string(),
            A =>            "0110000".to_string(),
            M =>            "1110000".to_string(),
            NotD =>         "0001101".to_string(),
            NotA =>         "0110011".to_string(),
            NotM =>         "1110001".to_string(),
            MinusD =>       "0001111".to_string(),
            MinusA =>       "0110011".to_string(),
            MinusM =>       "1110011".to_string(),
            DPlusOne =>     "0011111".to_string(),
            APlusOne =>     "0110111".to_string(),
            MPlusOne =>     "1110111".to_string(),
            DMinusOne =>    "0001110".to_string(),
            AMinusOne =>    "0110010".to_string(),
            MMinusOne =>    "1110010".to_string(),
            DPlusA =>       "0000010".to_string(),
            DPlusM =>       "1000010".to_string(),
            DMinusA =>      "0010011".to_string(),
            DMinusM =>      "1010011".to_string(),
            AMinusD =>      "0000111".to_string(),
            MMinusD =>      "1000111".to_string(),
            DAndA =>        "0000000".to_string(),
            DAndM =>        "1000000".to_string(),
            DOrA =>         "0010101".to_string(),
            DOrM =>         "1010101".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Jump {
    Null,
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,
}

impl Token<Jump> for Jump {
    fn new(jump: Option<String>) -> Result<Jump> {
        use Jump::*;
        match jump {
            None => Ok(Null),
            Some(j) => {
                match &j[..] {
                    "JGT" => Ok(JGT),
                    "JEQ" => Ok(JEQ),
                    "JGE" => Ok(JGE),
                    "JLT" => Ok(JLT),
                    "JNE" => Ok(JNE),
                    "JLE" => Ok(JLE),
                    "JMP" => Ok(JMP),
                    _ => Err(anyhow!("unexpected jump token: {}", j)),
                }
            }
        }
    }
    fn to_bin(&self) -> String {
        use Jump::*;
        match &self {
            Null => "000".to_string(),
            JGT =>  "001".to_string(),
            JEQ =>  "010".to_string(),
            JGE =>  "011".to_string(),
            JLT =>  "100".to_string(),
            JNE =>  "101".to_string(),
            JLE =>  "110".to_string(),
            JMP =>  "111".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Line {
    Whitespace(String),
    A {
        address: u16,
    },
    C {
        dest: Dest,
        comp: Comp,
        jump: Jump,
    },
}

impl Line {
    pub fn new(line: &String) -> Result<Line> {
        if line.starts_with("//") || line == "" {
            Ok(Line::Whitespace(line.clone()))
        }
        else if line.starts_with("@") {
            let address = line[1..].parse()?;
            Ok(Line::A { address })
        } else {
            // we can safely unwrap throughout here because there will always
            // be at least a first element
            let split_by_comment: Vec<&str> = line.split("//").collect();
            let no_comment = split_by_comment.get(0).unwrap().trim();
            let split_by_jump: Vec<&str> = no_comment.split(";").collect();

            let no_jump = split_by_jump.get(0).unwrap().trim();
            let jump = match split_by_jump.get(1) {
                Some(s) => Some(s.to_string()),
                None => None,
            };

            let dest_comp: Vec<&str> = no_jump.split("=").collect();

            let comp: Option<String>;
            let dest: Option<String>;
            if dest_comp.len() == 2 {
                dest = Some(dest_comp.get(0).unwrap().to_string());
                comp = Some(dest_comp.get(1).unwrap().to_string());
            } else {
                dest = None;
                comp = Some(dest_comp.get(0).unwrap().to_string());
            }

            Ok(Line::C {
                dest: Dest::new(dest)?,
                comp: Comp::new(comp)?,
                jump: Jump::new(jump)?,
            })
        }
    }

    pub fn to_bin(&self) -> Result<String> {
        use Line::*;
        match &self {
            // this is gross, the padding includes '0b' at the beginning, so we
            // format 2 extra chars and just nuke them
            A {address} => Ok(format!("{:#018b}", address).replace("0b", "")),
            C {dest, comp, jump} => Ok(format!(
                "111{}{}{}",
                comp.to_bin(),
                dest.to_bin(),
                jump.to_bin(),
            )),
            _ => Err(anyhow!("No valid bin representation of this type exists: {:?}", &self)),
        }
    }
}
