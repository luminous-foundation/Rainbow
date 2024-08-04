// TODO: enum type
#[derive(Debug, Clone)]
#[repr(u8)]
pub enum Types {
    VOID    = 0x00,
    I8      = 0x01,
    I16     = 0x02,
    I32     = 0x03,
    I64     = 0x04,
    U8      = 0x05,
    U16     = 0x06,
    U32     = 0x07,
    U64     = 0x08,
    F16     = 0x09,
    F32     = 0x0A,
    F64     = 0x0B,
    POINTER = 0x0C,
    TYPE    = 0x0D,
    STRUCT  = 0x0E,
    NAME    = 0x0F,
}

impl Types {
    pub fn from_u8(typ: u8) -> Types {
        match typ {
            0x00 => Types::VOID,
            0x01 => Types::I8,
            0x02 => Types::I16,
            0x03 => Types::I32,
            0x04 => Types::I64,
            0x05 => Types::U8,
            0x06 => Types::U16,
            0x07 => Types::U32,
            0x08 => Types::U64,
            0x09 => Types::F16,
            0x0A => Types::F32,
            0x0B => Types::F64,
            0x0C => Types::POINTER,
            0x0D => Types::TYPE,
            0x0E => Types::STRUCT,
            0x0F => Types::NAME,
            _ => panic!("unknown type {:#04x}", typ)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Type {
    pub typ: Vec<Types>,
}