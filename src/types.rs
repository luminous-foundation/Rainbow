use std::ops::{Add, Div, Mul, Sub};
use half::f16;

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum Types {
    Void(u8)         = 0x00,
    I8(i8)           = 0x01,
    I16(i16)         = 0x02,
    I32(i32)         = 0x03,
    I64(i64)         = 0x04,
    U8(u8)           = 0x05,
    U16(u16)         = 0x06,
    U32(u32)         = 0x07,
    U64(u64)         = 0x08,
    F16(f16)         = 0x09,
    F32(f32)         = 0x0A,
    F64(f64)         = 0x0B,
    Pointer(u64)     = 0x0C, // 32 bit machines not allowed
    Type(u8)         = 0x0D,
    Struct(u8)       = 0x0E, // TODO: this should have a special type but this is just here to fill it
    Function(String) = 0x0F
}

macro_rules! impl_add_for_numeric {
    ($($t:ty),*) => { // voodoo magic
        $(
            impl Add<$t> for Types {
                type Output = Self;

                fn add(self, other: $t) -> Self {
                    match self {
                        Types::I8(a)  => Types::I8(a.wrapping_add(other as i8)),
                        Types::I16(a) => Types::I16(a.wrapping_add(other as i16)),
                        Types::I32(a) => Types::I32(a.wrapping_add(other as i32)),
                        Types::I64(a) => Types::I64(a.wrapping_add(other as i64)),
                        Types::U8(a)  => Types::U8(a.wrapping_add(other as u8)),
                        Types::U16(a) => Types::U16(a.wrapping_add(other as u16)),
                        Types::U32(a) => Types::U32(a.wrapping_add(other as u32)),
                        Types::U64(a) => Types::U64(a.wrapping_add(other as u64)),
                        Types::F16(a) => Types::F16(a + f16::from_f32(other as f32)),
                        Types::F32(a) => Types::F32(a + other as f32),
                        Types::F64(a) => Types::F64(a + other as f64),
                        _ => panic!("Unsupported types for addition"),
                    }
                }
            }
        )*
    };
}

macro_rules! impl_sub_for_numeric {
    ($($t:ty),*) => { // voodoo magic
        $(
            impl Sub<$t> for Types {
                type Output = Self;

                fn sub(self, other: $t) -> Self {
                    match self {
                        Types::I8(a)  => Types::I8(a.wrapping_sub(other as i8)),
                        Types::I16(a) => Types::I16(a.wrapping_sub(other as i16)),
                        Types::I32(a) => Types::I32(a.wrapping_sub(other as i32)),
                        Types::I64(a) => Types::I64(a.wrapping_sub(other as i64)),
                        Types::U8(a)  => Types::U8(a.wrapping_sub(other as u8)),
                        Types::U16(a) => Types::U16(a.wrapping_sub(other as u16)),
                        Types::U32(a) => Types::U32(a.wrapping_sub(other as u32)),
                        Types::U64(a) => Types::U64(a.wrapping_sub(other as u64)),
                        Types::F16(a) => Types::F16(a - f16::from_f32(other as f32)),
                        Types::F32(a) => Types::F32(a - other as f32),
                        Types::F64(a) => Types::F64(a - other as f64),
                        _ => panic!("Unsupported types for subtraction"),
                    }
                }
            }
        )*
    };
}

macro_rules! impl_mul_for_numeric {
    ($($t:ty),*) => { // voodoo magic
        $(
            impl Mul<$t> for Types {
                type Output = Self;

                fn mul(self, other: $t) -> Self {
                    match self {
                        Types::I8(a)  => Types::I8(a.wrapping_mul(other as i8)),
                        Types::I16(a) => Types::I16(a.wrapping_mul(other as i16)),
                        Types::I32(a) => Types::I32(a.wrapping_mul(other as i32)),
                        Types::I64(a) => Types::I64(a.wrapping_mul(other as i64)),
                        Types::U8(a)  => Types::U8(a.wrapping_mul(other as u8)),
                        Types::U16(a) => Types::U16(a.wrapping_mul(other as u16)),
                        Types::U32(a) => Types::U32(a.wrapping_mul(other as u32)),
                        Types::U64(a) => Types::U64(a.wrapping_mul(other as u64)),
                        Types::F16(a) => Types::F16(a * f16::from_f32(other as f32)),
                        Types::F32(a) => Types::F32(a * other as f32),
                        Types::F64(a) => Types::F64(a * other as f64),
                        _ => panic!("Unsupported types for multiplication"),
                    }
                }
            }
        )*
    };
}

macro_rules! impl_div_for_numeric {
    ($($t:ty),*) => { // voodoo magic
        $(
            impl Div<$t> for Types {
                type Output = Self;

                fn div(self, other: $t) -> Self {
                    match self {
                        Types::I8(a)  => Types::I8(a.wrapping_div(other as i8)),
                        Types::I16(a) => Types::I16(a.wrapping_div(other as i16)),
                        Types::I32(a) => Types::I32(a.wrapping_div(other as i32)),
                        Types::I64(a) => Types::I64(a.wrapping_div(other as i64)),
                        Types::U8(a)  => Types::U8(a.wrapping_div(other as u8)),
                        Types::U16(a) => Types::U16(a.wrapping_div(other as u16)),
                        Types::U32(a) => Types::U32(a.wrapping_div(other as u32)),
                        Types::U64(a) => Types::U64(a.wrapping_div(other as u64)),
                        Types::F16(a) => Types::F16(a / f16::from_f32(other as f32)),
                        Types::F32(a) => Types::F32(a / other as f32),
                        Types::F64(a) => Types::F64(a / other as f64),
                        _ => panic!("Unsupported types for division"),
                    }
                }
            }
        )*
    };
}

impl_add_for_numeric!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);
impl_sub_for_numeric!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);
impl_mul_for_numeric!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);
impl_div_for_numeric!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);



// FIXME: make these work for all types
impl Add<f16> for Types {
    type Output = Self;

    fn add(self, other: f16) -> Self {
        match self {
            Types::F16(a) => Types::F16(a + other),
            _ => panic!("Unsupported types for addition"),
        }
    }
}

impl Sub<f16> for Types {
    type Output = Self;

    fn sub(self, other: f16) -> Self {
        match self {
            Types::F16(a) => Types::F16(a - other),
            _ => panic!("Unsupported types for subtraction"),
        }
    }
}

impl Mul<f16> for Types {
    type Output = Self;

    fn mul(self, other: f16) -> Self {
        match self {
            Types::F16(a) => Types::F16(a * other),
            _ => panic!("Unsupported types for multiplication"),
        }
    }
}

impl Div<f16> for Types {
    type Output = Self;

    fn div(self, other: f16) -> Self {
        match self {
            Types::F16(a) => Types::F16(a / other),
            _ => panic!("Unsupported types for division"),
        }
    }
}

impl Add<Types> for Types {
    type Output = Self;

    fn add(self, other: Types) -> Self {
        match (self, other) {
            (Types::Void(_), _) | (_, Types::Void(_))
            | (Types::Pointer(_), _) | (_, Types::Pointer(_))
            | (Types::Type(_), _) | (_, Types::Type(_))
            | (Types::Struct(_), _) | (_, Types::Struct(_))
            | (Types::Function(_), _) | (_, Types::Function(_)) => panic!("Unsupported types for addition"),
            (a, Types::I8(b))   => a + b,
            (a, Types::I16(b)) => a + b,
            (a, Types::I32(b)) => a + b,
            (a, Types::I64(b)) => a + b,
            (a, Types::U8(b))   => a + b,
            (a, Types::U16(b)) => a + b,
            (a, Types::U32(b)) => a + b,
            (a, Types::U64(b)) => a + b,
            (a, Types::F16(b)) => a + b,
            (a, Types::F32(b)) => a + b,
            (a, Types::F64(b)) => a + b,
        }
    }
}

impl Sub<Types> for Types {
    type Output = Self;

    fn sub(self, other: Types) -> Self {
        match (self, other) {
            (Types::Void(_), _) | (_, Types::Void(_))
            | (Types::Pointer(_), _) | (_, Types::Pointer(_))
            | (Types::Type(_), _) | (_, Types::Type(_))
            | (Types::Struct(_), _) | (_, Types::Struct(_))
            | (Types::Function(_), _) | (_, Types::Function(_)) => panic!("Unsupported types for addition"),
            (a, Types::I8(b))   => a - b,
            (a, Types::I16(b)) => a - b,
            (a, Types::I32(b)) => a - b,
            (a, Types::I64(b)) => a - b,
            (a, Types::U8(b))   => a - b,
            (a, Types::U16(b)) => a - b,
            (a, Types::U32(b)) => a - b,
            (a, Types::U64(b)) => a - b,
            (a, Types::F16(b)) => a - b,
            (a, Types::F32(b)) => a - b,
            (a, Types::F64(b)) => a - b,
        }
    }
}

impl Mul<Types> for Types {
    type Output = Self;

    fn mul(self, other: Types) -> Self {
        match (self, other) {
            (Types::Void(_), _) | (_, Types::Void(_))
            | (Types::Pointer(_), _) | (_, Types::Pointer(_))
            | (Types::Type(_), _) | (_, Types::Type(_))
            | (Types::Struct(_), _) | (_, Types::Struct(_))
            | (Types::Function(_), _) | (_, Types::Function(_)) => panic!("Unsupported types for addition"),
            (a, Types::I8(b))   => a * b,
            (a, Types::I16(b)) => a * b,
            (a, Types::I32(b)) => a * b,
            (a, Types::I64(b)) => a * b,
            (a, Types::U8(b))   => a * b,
            (a, Types::U16(b)) => a * b,
            (a, Types::U32(b)) => a * b,
            (a, Types::U64(b)) => a * b,
            (a, Types::F16(b)) => a * b,
            (a, Types::F32(b)) => a * b,
            (a, Types::F64(b)) => a * b,
        }
    }
}

impl Div<Types> for Types {
    type Output = Self;

    fn div(self, other: Types) -> Self {
        match (self, other) {
            (Types::Void(_), _) | (_, Types::Void(_))
            | (Types::Pointer(_), _) | (_, Types::Pointer(_))
            | (Types::Type(_), _) | (_, Types::Type(_))
            | (Types::Struct(_), _) | (_, Types::Struct(_))
            | (Types::Function(_), _) | (_, Types::Function(_)) => panic!("Unsupported types for addition"),
            (a, Types::I8(b))   => a / b,
            (a, Types::I16(b)) => a / b,
            (a, Types::I32(b)) => a / b,
            (a, Types::I64(b)) => a / b,
            (a, Types::U8(b))   => a / b,
            (a, Types::U16(b)) => a / b,
            (a, Types::U32(b)) => a / b,
            (a, Types::U64(b)) => a / b,
            (a, Types::F16(b)) => a / b,
            (a, Types::F32(b)) => a / b,
            (a, Types::F64(b)) => a / b,
        }
    }
}

pub fn add(a: &Box<Types>, b: &Box<Types>) -> Box<Types> {
    let result = (**a).clone() + (**b).clone();
    Box::new(result)
}

pub fn sub(a: &Box<Types>, b: &Box<Types>) -> Box<Types> {
    let result = (**a).clone() - (**b).clone();
    Box::new(result)
}

pub fn mul(a: &Box<Types>, b: &Box<Types>) -> Box<Types> {
    let result = (**a).clone() * (**b).clone();
    Box::new(result)
}

pub fn div(a: &Box<Types>, b: &Box<Types>) -> Box<Types> {
    let result = (**a).clone() / (**b).clone();
    Box::new(result)
}

pub fn parse_imm(program: &Vec<u8>, pc: &mut usize) -> Box<Types> {
    let t = program[*pc];
    *pc += 1;

    match t {
        0x00 | 0x0E => {
            panic!("cannot parse type {}", format!("0x{:02x}", t));
        }
        0x01 => {
            let ret = Box::new(Types::I8(program[*pc] as i8));
            *pc += 1;
            return ret;
        }
        0x02 => {
            let ret = Box::new(Types::I16(i16::from_ne_bytes({
                program[*pc..*pc+2].try_into().unwrap()
            })));
            *pc += 2;
            return ret;
        }
        0x03 => {
            let ret = Box::new(Types::I32(i32::from_ne_bytes({
                program[*pc..*pc+4].try_into().unwrap()
            })));
            *pc += 4;
            return ret;
        }
        0x04 => {
            let ret = Box::new(Types::I64(i64::from_ne_bytes({
                program[*pc..*pc+8].try_into().unwrap()
            })));
            *pc += 8;
            return ret;
        }
        
        0x05 => {
            let ret = Box::new(Types::U8(program[*pc] as u8));
            *pc += 1;
            return ret;
        }
        0x06 => {
            let ret = Box::new(Types::U16(u16::from_ne_bytes({
                program[*pc..*pc+2].try_into().unwrap()
            })));
            *pc += 2;
            return ret;
        }
        0x07 => {
            let ret = Box::new(Types::U32(u32::from_ne_bytes({
                program[*pc..*pc+4].try_into().unwrap()
            })));
            *pc += 4;
            return ret;
        }
        0x08 => {
            let ret = Box::new(Types::U64(u64::from_ne_bytes({
                program[*pc..*pc+8].try_into().unwrap()
            })));
            *pc += 8;
            return ret;
        }

        0x09 => {
            let ret = Box::new(Types::F16(f16::from_bits(
                u16::from_ne_bytes(
                    program[*pc..*pc+2].try_into().unwrap()
                )
            )));
            *pc += 2;
            return ret;
        }
        0x0A => {
            let ret = Box::new(Types::F32(f32::from_ne_bytes({
                program[*pc..*pc+4].try_into().unwrap()
            })));
            *pc += 4;
            return ret;
        }
        0x0B => {
            let ret = Box::new(Types::F64(f64::from_ne_bytes({
                program[*pc..*pc+8].try_into().unwrap()
            })));
            *pc += 8;
            return ret;
        }
        
        0x0C => {
            let ret = Box::new(Types::Pointer(u64::from_ne_bytes({
                program[*pc..*pc+4].try_into().unwrap()
            })));
            *pc += 8;
            return ret;
        }

        0x0D => {
            let ret = Box::new(Types::Type(program[*pc] as u8));
            *pc += 1;
            return ret;
        }

        0x0F => {
            let length = program[*pc] as usize;
            *pc += 1;
            let ret = Box::new(Types::Function(String::from_utf8(program[*pc..(*pc+length)].try_into().unwrap()).unwrap()));
            *pc += length;
            return ret;
        }

        _ => {
            panic!("unknown type {}", format!("0x{:02x}", t));
        }
    }
}

pub fn from_type(t: u8) -> Box<Types> {
    match t {
        0x00 => Box::new(Types::Void(0)),
        0x01 => Box::new(Types::I8(0)),
        0x02 => Box::new(Types::I16(0)),
        0x03 => Box::new(Types::I32(0)),
        0x04 => Box::new(Types::I64(0)),
        0x05 => Box::new(Types::U8(0)),
        0x06 => Box::new(Types::U16(0)),
        0x07 => Box::new(Types::U32(0)),
        0x08 => Box::new(Types::U64(0)),
        0x09 => Box::new(Types::F16(f16::from_f32(0.0))),
        0x0A => Box::new(Types::F32(0.0)),
        0x0B => Box::new(Types::F64(0.0)),
        0x0C => Box::new(Types::Pointer(0)),
        0x0D => Box::new(Types::Type(0)),
        0x0E => Box::new(Types::Struct(0)),
        0x0F => Box::new(Types::Function(String::from(""))),
        _ => {
            panic!("unknown type {}", format!("0x{:02x}", t));
        }
    }
}

// do not open function
pub fn cast_type(val: Box<Types>, t: u8) -> Box<Types> {
    match (val.as_ref(), t) {
        (Types::Void(v), 0x00) => Box::new(Types::Void(*v as u8)),
        (Types::Void(v), 0x01) => Box::new(Types::I8(*v as i8)),
        (Types::Void(v), 0x02) => Box::new(Types::I16(*v as i16)),
        (Types::Void(v), 0x03) => Box::new(Types::I32(*v as i32)),
        (Types::Void(v), 0x04) => Box::new(Types::I64(*v as i64)),
        (Types::Void(v), 0x05) => Box::new(Types::U8(*v as u8)),
        (Types::Void(v), 0x06) => Box::new(Types::U16(*v as u16)),
        (Types::Void(v), 0x07) => Box::new(Types::U32(*v as u32)),
        (Types::Void(v), 0x08) => Box::new(Types::U64(*v as u64)),
        (Types::Void(v), 0x09) => Box::new(Types::F16(f16::from_f32(*v as f32))),
        (Types::Void(v), 0x0A) => Box::new(Types::F32(*v as f32)),
        (Types::Void(v), 0x0B) => Box::new(Types::F64(*v as f64)),
        (Types::Void(v), 0x0C) => Box::new(Types::Pointer(*v as u64)),
        (Types::Void(v), 0x0D) => Box::new(Types::Type(*v as u8)),
        (Types::I8(v), 0x00) => Box::new(Types::Void(*v as u8)),
        (Types::I8(v), 0x01) => Box::new(Types::I8(*v as i8)),
        (Types::I8(v), 0x02) => Box::new(Types::I16(*v as i16)),
        (Types::I8(v), 0x03) => Box::new(Types::I32(*v as i32)),
        (Types::I8(v), 0x04) => Box::new(Types::I64(*v as i64)),
        (Types::I8(v), 0x05) => Box::new(Types::U8(*v as u8)),
        (Types::I8(v), 0x06) => Box::new(Types::U16(*v as u16)),
        (Types::I8(v), 0x07) => Box::new(Types::U32(*v as u32)),
        (Types::I8(v), 0x08) => Box::new(Types::U64(*v as u64)),
        (Types::I8(v), 0x09) => Box::new(Types::F16(f16::from_f32(*v as f32))),
        (Types::I8(v), 0x0A) => Box::new(Types::F32(*v as f32)),
        (Types::I8(v), 0x0B) => Box::new(Types::F64(*v as f64)),
        (Types::I8(v), 0x0C) => Box::new(Types::Pointer(*v as u64)),
        (Types::I8(v), 0x0D) => Box::new(Types::Type(*v as u8)),
        (Types::I16(v), 0x00) => Box::new(Types::Void(*v as u8)),
        (Types::I16(v), 0x01) => Box::new(Types::I8(*v as i8)),
        (Types::I16(v), 0x02) => Box::new(Types::I16(*v as i16)),
        (Types::I16(v), 0x03) => Box::new(Types::I32(*v as i32)),
        (Types::I16(v), 0x04) => Box::new(Types::I64(*v as i64)),
        (Types::I16(v), 0x05) => Box::new(Types::U8(*v as u8)),
        (Types::I16(v), 0x06) => Box::new(Types::U16(*v as u16)),
        (Types::I16(v), 0x07) => Box::new(Types::U32(*v as u32)),
        (Types::I16(v), 0x08) => Box::new(Types::U64(*v as u64)),
        (Types::I16(v), 0x09) => Box::new(Types::F16(f16::from_f32(*v as f32))),
        (Types::I16(v), 0x0A) => Box::new(Types::F32(*v as f32)),
        (Types::I16(v), 0x0B) => Box::new(Types::F64(*v as f64)),
        (Types::I16(v), 0x0C) => Box::new(Types::Pointer(*v as u64)),
        (Types::I16(v), 0x0D) => Box::new(Types::Type(*v as u8)),
        (Types::I32(v), 0x00) => Box::new(Types::Void(*v as u8)),
        (Types::I32(v), 0x01) => Box::new(Types::I8(*v as i8)),
        (Types::I32(v), 0x02) => Box::new(Types::I16(*v as i16)),
        (Types::I32(v), 0x03) => Box::new(Types::I32(*v as i32)),
        (Types::I32(v), 0x04) => Box::new(Types::I64(*v as i64)),
        (Types::I32(v), 0x05) => Box::new(Types::U8(*v as u8)),
        (Types::I32(v), 0x06) => Box::new(Types::U16(*v as u16)),
        (Types::I32(v), 0x07) => Box::new(Types::U32(*v as u32)),
        (Types::I32(v), 0x08) => Box::new(Types::U64(*v as u64)),
        (Types::I32(v), 0x09) => Box::new(Types::F16(f16::from_f32(*v as f32))),
        (Types::I32(v), 0x0A) => Box::new(Types::F32(*v as f32)),
        (Types::I32(v), 0x0B) => Box::new(Types::F64(*v as f64)),
        (Types::I32(v), 0x0C) => Box::new(Types::Pointer(*v as u64)),
        (Types::I32(v), 0x0D) => Box::new(Types::Type(*v as u8)),
        (Types::I64(v), 0x00) => Box::new(Types::Void(*v as u8)),
        (Types::I64(v), 0x01) => Box::new(Types::I8(*v as i8)),
        (Types::I64(v), 0x02) => Box::new(Types::I16(*v as i16)),
        (Types::I64(v), 0x03) => Box::new(Types::I32(*v as i32)),
        (Types::I64(v), 0x04) => Box::new(Types::I64(*v as i64)),
        (Types::I64(v), 0x05) => Box::new(Types::U8(*v as u8)),
        (Types::I64(v), 0x06) => Box::new(Types::U16(*v as u16)),
        (Types::I64(v), 0x07) => Box::new(Types::U32(*v as u32)),
        (Types::I64(v), 0x08) => Box::new(Types::U64(*v as u64)),
        (Types::I64(v), 0x09) => Box::new(Types::F16(f16::from_f32(*v as f32))),
        (Types::I64(v), 0x0A) => Box::new(Types::F32(*v as f32)),
        (Types::I64(v), 0x0B) => Box::new(Types::F64(*v as f64)),
        (Types::I64(v), 0x0C) => Box::new(Types::Pointer(*v as u64)),
        (Types::I64(v), 0x0D) => Box::new(Types::Type(*v as u8)),
        (Types::U8(v), 0x00) => Box::new(Types::Void(*v as u8)),
        (Types::U8(v), 0x01) => Box::new(Types::I8(*v as i8)),
        (Types::U8(v), 0x02) => Box::new(Types::I16(*v as i16)),
        (Types::U8(v), 0x03) => Box::new(Types::I32(*v as i32)),
        (Types::U8(v), 0x04) => Box::new(Types::I64(*v as i64)),
        (Types::U8(v), 0x05) => Box::new(Types::U8(*v as u8)),
        (Types::U8(v), 0x06) => Box::new(Types::U16(*v as u16)),
        (Types::U8(v), 0x07) => Box::new(Types::U32(*v as u32)),
        (Types::U8(v), 0x08) => Box::new(Types::U64(*v as u64)),
        (Types::U8(v), 0x09) => Box::new(Types::F16(f16::from_f32(*v as f32))),
        (Types::U8(v), 0x0A) => Box::new(Types::F32(*v as f32)),
        (Types::U8(v), 0x0B) => Box::new(Types::F64(*v as f64)),
        (Types::U8(v), 0x0C) => Box::new(Types::Pointer(*v as u64)),
        (Types::U8(v), 0x0D) => Box::new(Types::Type(*v as u8)),
        (Types::U16(v), 0x00) => Box::new(Types::Void(*v as u8)),
        (Types::U16(v), 0x01) => Box::new(Types::I8(*v as i8)),
        (Types::U16(v), 0x02) => Box::new(Types::I16(*v as i16)),
        (Types::U16(v), 0x03) => Box::new(Types::I32(*v as i32)),
        (Types::U16(v), 0x04) => Box::new(Types::I64(*v as i64)),
        (Types::U16(v), 0x05) => Box::new(Types::U8(*v as u8)),
        (Types::U16(v), 0x06) => Box::new(Types::U16(*v as u16)),
        (Types::U16(v), 0x07) => Box::new(Types::U32(*v as u32)),
        (Types::U16(v), 0x08) => Box::new(Types::U64(*v as u64)),
        (Types::U16(v), 0x09) => Box::new(Types::F16(f16::from_f32(*v as f32))),
        (Types::U16(v), 0x0A) => Box::new(Types::F32(*v as f32)),
        (Types::U16(v), 0x0B) => Box::new(Types::F64(*v as f64)),
        (Types::U16(v), 0x0C) => Box::new(Types::Pointer(*v as u64)),
        (Types::U16(v), 0x0D) => Box::new(Types::Type(*v as u8)),
        (Types::U32(v), 0x00) => Box::new(Types::Void(*v as u8)),
        (Types::U32(v), 0x01) => Box::new(Types::I8(*v as i8)),
        (Types::U32(v), 0x02) => Box::new(Types::I16(*v as i16)),
        (Types::U32(v), 0x03) => Box::new(Types::I32(*v as i32)),
        (Types::U32(v), 0x04) => Box::new(Types::I64(*v as i64)),
        (Types::U32(v), 0x05) => Box::new(Types::U8(*v as u8)),
        (Types::U32(v), 0x06) => Box::new(Types::U16(*v as u16)),
        (Types::U32(v), 0x07) => Box::new(Types::U32(*v as u32)),
        (Types::U32(v), 0x08) => Box::new(Types::U64(*v as u64)),
        (Types::U32(v), 0x09) => Box::new(Types::F16(f16::from_f32(*v as f32))),
        (Types::U32(v), 0x0A) => Box::new(Types::F32(*v as f32)),
        (Types::U32(v), 0x0B) => Box::new(Types::F64(*v as f64)),
        (Types::U32(v), 0x0C) => Box::new(Types::Pointer(*v as u64)),
        (Types::U32(v), 0x0D) => Box::new(Types::Type(*v as u8)),
        (Types::U64(v), 0x00) => Box::new(Types::Void(*v as u8)),
        (Types::U64(v), 0x01) => Box::new(Types::I8(*v as i8)),
        (Types::U64(v), 0x02) => Box::new(Types::I16(*v as i16)),
        (Types::U64(v), 0x03) => Box::new(Types::I32(*v as i32)),
        (Types::U64(v), 0x04) => Box::new(Types::I64(*v as i64)),
        (Types::U64(v), 0x05) => Box::new(Types::U8(*v as u8)),
        (Types::U64(v), 0x06) => Box::new(Types::U16(*v as u16)),
        (Types::U64(v), 0x07) => Box::new(Types::U32(*v as u32)),
        (Types::U64(v), 0x08) => Box::new(Types::U64(*v as u64)),
        (Types::U64(v), 0x09) => Box::new(Types::F16(f16::from_f32(*v as f32))),
        (Types::U64(v), 0x0A) => Box::new(Types::F32(*v as f32)),
        (Types::U64(v), 0x0B) => Box::new(Types::F64(*v as f64)),
        (Types::U64(v), 0x0C) => Box::new(Types::Pointer(*v as u64)),
        (Types::U64(v), 0x0D) => Box::new(Types::Type(*v as u8)),
        (Types::F16(v), 0x00) => Box::new(Types::Void((*v).to_f32() as u8)),
        (Types::F16(v), 0x01) => Box::new(Types::I8((*v).to_f32() as i8)),
        (Types::F16(v), 0x02) => Box::new(Types::I16((*v).to_f32() as i16)),
        (Types::F16(v), 0x03) => Box::new(Types::I32((*v).to_f32() as i32)),
        (Types::F16(v), 0x04) => Box::new(Types::I64((*v).to_f32() as i64)),
        (Types::F16(v), 0x05) => Box::new(Types::U8((*v).to_f32() as u8)),
        (Types::F16(v), 0x06) => Box::new(Types::U16((*v).to_f32() as u16)),
        (Types::F16(v), 0x07) => Box::new(Types::U32((*v).to_f32() as u32)),
        (Types::F16(v), 0x08) => Box::new(Types::U64((*v).to_f32() as u64)),
        (Types::F16(v), 0x09) => Box::new(Types::F16(*v)),
        (Types::F16(v), 0x0A) => Box::new(Types::F32((*v).to_f32())),
        (Types::F16(v), 0x0B) => Box::new(Types::F64((*v).to_f64())),
        (Types::F16(v), 0x0C) => Box::new(Types::Pointer((*v).to_f32() as u64)),
        (Types::F16(v), 0x0D) => Box::new(Types::Type((*v).to_f32() as u8)),
        (Types::F32(v), 0x00) => Box::new(Types::Void(*v as u8)),
        (Types::F32(v), 0x01) => Box::new(Types::I8(*v as i8)),
        (Types::F32(v), 0x02) => Box::new(Types::I16(*v as i16)),
        (Types::F32(v), 0x03) => Box::new(Types::I32(*v as i32)),
        (Types::F32(v), 0x04) => Box::new(Types::I64(*v as i64)),
        (Types::F32(v), 0x05) => Box::new(Types::U8(*v as u8)),
        (Types::F32(v), 0x06) => Box::new(Types::U16(*v as u16)),
        (Types::F32(v), 0x07) => Box::new(Types::U32(*v as u32)),
        (Types::F32(v), 0x08) => Box::new(Types::U64(*v as u64)),
        (Types::F32(v), 0x09) => Box::new(Types::F16(f16::from_f32(*v as f32))),
        (Types::F32(v), 0x0A) => Box::new(Types::F32(*v as f32)),
        (Types::F32(v), 0x0B) => Box::new(Types::F64(*v as f64)),
        (Types::F32(v), 0x0C) => Box::new(Types::Pointer(*v as u64)),
        (Types::F32(v), 0x0D) => Box::new(Types::Type(*v as u8)),
        (Types::F64(v), 0x00) => Box::new(Types::Void(*v as u8)),
        (Types::F64(v), 0x01) => Box::new(Types::I8(*v as i8)),
        (Types::F64(v), 0x02) => Box::new(Types::I16(*v as i16)),
        (Types::F64(v), 0x03) => Box::new(Types::I32(*v as i32)),
        (Types::F64(v), 0x04) => Box::new(Types::I64(*v as i64)),
        (Types::F64(v), 0x05) => Box::new(Types::U8(*v as u8)),
        (Types::F64(v), 0x06) => Box::new(Types::U16(*v as u16)),
        (Types::F64(v), 0x07) => Box::new(Types::U32(*v as u32)),
        (Types::F64(v), 0x08) => Box::new(Types::U64(*v as u64)),
        (Types::F64(v), 0x09) => Box::new(Types::F16(f16::from_f32(*v as f32))),
        (Types::F64(v), 0x0A) => Box::new(Types::F32(*v as f32)),
        (Types::F64(v), 0x0B) => Box::new(Types::F64(*v as f64)),
        (Types::F64(v), 0x0C) => Box::new(Types::Pointer(*v as u64)),
        (Types::F64(v), 0x0D) => Box::new(Types::Type(*v as u8)),
        (Types::Pointer(v), 0x00) => Box::new(Types::Void(*v as u8)),
        (Types::Pointer(v), 0x01) => Box::new(Types::I8(*v as i8)),
        (Types::Pointer(v), 0x02) => Box::new(Types::I16(*v as i16)),
        (Types::Pointer(v), 0x03) => Box::new(Types::I32(*v as i32)),
        (Types::Pointer(v), 0x04) => Box::new(Types::I64(*v as i64)),
        (Types::Pointer(v), 0x05) => Box::new(Types::U8(*v as u8)),
        (Types::Pointer(v), 0x06) => Box::new(Types::U16(*v as u16)),
        (Types::Pointer(v), 0x07) => Box::new(Types::U32(*v as u32)),
        (Types::Pointer(v), 0x08) => Box::new(Types::U64(*v as u64)),
        (Types::Pointer(v), 0x09) => Box::new(Types::F16(f16::from_f32(*v as f32))),
        (Types::Pointer(v), 0x0A) => Box::new(Types::F32(*v as f32)),
        (Types::Pointer(v), 0x0B) => Box::new(Types::F64(*v as f64)),
        (Types::Pointer(v), 0x0C) => Box::new(Types::Pointer(*v as u64)),
        (Types::Pointer(v), 0x0D) => Box::new(Types::Type(*v as u8)),
        (Types::Type(v), 0x00) => Box::new(Types::Void(*v as u8)),
        (Types::Type(v), 0x01) => Box::new(Types::I8(*v as i8)),
        (Types::Type(v), 0x02) => Box::new(Types::I16(*v as i16)),
        (Types::Type(v), 0x03) => Box::new(Types::I32(*v as i32)),
        (Types::Type(v), 0x04) => Box::new(Types::I64(*v as i64)),
        (Types::Type(v), 0x05) => Box::new(Types::U8(*v as u8)),
        (Types::Type(v), 0x06) => Box::new(Types::U16(*v as u16)),
        (Types::Type(v), 0x07) => Box::new(Types::U32(*v as u32)),
        (Types::Type(v), 0x08) => Box::new(Types::U64(*v as u64)),
        (Types::Type(v), 0x09) => Box::new(Types::F16(f16::from_f32(*v as f32))),
        (Types::Type(v), 0x0A) => Box::new(Types::F32(*v as f32)),
        (Types::Type(v), 0x0B) => Box::new(Types::F64(*v as f64)),
        (Types::Type(v), 0x0C) => Box::new(Types::Pointer(*v as u64)),
        (Types::Type(v), 0x0D) => Box::new(Types::Type(*v as u8)),
        _ => panic!("Unsupported type conversion"),
    }
}

