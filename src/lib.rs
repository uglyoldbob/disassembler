use omnom::ReadExt;

pub enum Architecture {
    PowerPc,
    Unsupported,
}

pub enum Address {
    Address8(u8),
    Address16(u16),
    Address32(u32),
    Address64(u64),
}

pub struct Disassembler {
    a: Architecture,
    data: std::io::Cursor<Vec<u8>>,
    address: Address,
}

impl Into<Architecture> for object::Architecture {
    fn into(self) -> Architecture {
        match self {
            object::Architecture::PowerPc => Architecture::PowerPc,
            _ => Architecture::Unsupported,
        }
    }
}

impl Disassembler {
    pub fn new<T: Into<Architecture>>(arch: T, d: &[u8], adr: Address) -> Self {
        Self {
            a: arch.into(),
            data: std::io::Cursor::new(d.to_vec()),
            address: adr,
        }
    }

    pub fn convert_next_instruction(&mut self) -> Option<u32> {
        match self.a {
            Architecture::PowerPc => {
                let i: u32 = self.data.read_le().ok()?;
                
            }
            _ => {
                unimplemented!();
            }
        }
        let v: u32 = self.data.read_le().ok()?;
        Some(v)
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
