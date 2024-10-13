use std::fmt;

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

    pub fn get_size(&self) -> usize {
        match self {
            Types::VOID => 0,
            Types::I8 => 1,
            Types::I16 => 2,
            Types::I32 => 4,
            Types::I64 => 8,
            Types::U8 => 1,
            Types::U16 => 2,
            Types::U32 => 4,
            Types::U64 => 8,
            Types::F16 => 2,
            Types::F32 => 4,
            Types::F64 => 8,
            Types::POINTER => std::mem::size_of::<usize>(),
            Types::TYPE => 1,
            Types::STRUCT => 0, // struct does not have a known size
            Types::NAME => 0, // struct does not have a known size
        }
    }
}

#[derive(Debug, Clone)]
pub struct Type {
    pub typ: Vec<Types>,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();

        let mut rev_typ = self.typ.clone();
        rev_typ.reverse();
        for typ in &self.typ {
            str += match typ {
                Types::VOID => "void",
                Types::I8 => "i8",
                Types::I16 => "i16",
                Types::I32 => "i32",
                Types::I64 => "i64",
                Types::U8 => "u8",
                Types::U16 => "u16",
                Types::U32 => "u32",
                Types::U64 => "u64",
                Types::F16 => "f16",
                Types::F32 => "f32",
                Types::F64 => "f64",
                Types::POINTER => "*",
                Types::TYPE => "type",
                Types::STRUCT => "struct",
                Types::NAME => "name",
            }
        }

        f.write_str(&str)
    }
}

impl Type {
    pub fn pop(self) -> Type {
        return Type { typ: self.typ[1..self.typ.len()].to_vec() };
    }

    pub fn get_size(&self) -> usize {
        return self.typ[0].get_size();
    }
}