use core::fmt;
use std::cmp::{self, Ordering};

use crate::_type::Type;

// TODO: enum type
#[derive(Debug, Clone)]
pub enum Values {
    VOID,
    // using basic wrappers to make type casting way easier
    SIGNED(i64),
    UNSIGNED(u64),
    DECIMAL(f64),
    POINTER(usize, usize), // index and size
    STRUCT(String, String, usize), // module, name, index
    TYPE(Type),
    NAME(String),
}

impl fmt::Display for Values {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();

        match &self {
            Values::VOID => res += "VOID",
            Values::SIGNED(n) => res += &n.to_string(),
            Values::UNSIGNED(n) => res += &n.to_string(),
            Values::DECIMAL(n) => res += &n.to_string(),
            Values::POINTER(n, _) => res += &("*".to_string() + &format!("0x{:x}", n)),
            Values::STRUCT(module, name, _) => {
                if module.len() > 0 {
                    res += &format!("struct({module}.{name})")
                } else {
                    res += &format!("struct({name})");
                }
            }
            Values::TYPE(t) => res += &format!("{t}"),
            Values::NAME(n) => {
                res += "\"";
                res += &n;
                res += "\"";
            }
        }

        f.write_str(&res)
    }
}

#[derive(Debug, Clone)]
pub struct Value {
    pub typ: Type,
    pub val: Values,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();

        res += &format!("{}", self.val);

        f.write_str(&res)
    }
}

impl Value {
    pub fn set(&mut self, other: &Values) { // yeah it's just a wrapper what about it
        self.val.set(other);
    }
}

macro_rules! math {
    ($self:expr, $other:expr, $op:tt, $op_name:expr, $op_plural:expr) => {
        return match($self, $other) {
            (Values::VOID, _) => Values::VOID,
            (Values::SIGNED(s), Values::VOID) => Values::SIGNED(*s),
            (Values::SIGNED(s), Values::SIGNED(v)) => Values::SIGNED(*s $op *v),
            (Values::SIGNED(s), Values::UNSIGNED(v)) => Values::SIGNED(*s $op *v as i64),
            (Values::SIGNED(s), Values::DECIMAL(v)) => Values::SIGNED(*s $op *v as i64),
            (Values::SIGNED(s), Values::POINTER(v, _)) => Values::SIGNED(*s $op *v as i64),
            (Values::SIGNED(_), Values::STRUCT(_, _, _)) => panic!("type mismatch: cannot {} values of type `struct` and `number`", $op_name),
            (Values::SIGNED(_), Values::TYPE(_)) => panic!("type mismatch: cannot {} values of type `type` and `number`", $op_name),
            (Values::SIGNED(_), Values::NAME(_)) => panic!("type mismatch: cannot {} values of type `name` and `number`", $op_name),
            (Values::UNSIGNED(s), Values::VOID) => Values::UNSIGNED(*s),
            (Values::UNSIGNED(s), Values::SIGNED(v)) => Values::UNSIGNED(*s $op *v as u64),
            (Values::UNSIGNED(s), Values::UNSIGNED(v)) => Values::UNSIGNED(*s $op *v),
            (Values::UNSIGNED(s), Values::DECIMAL(v)) => Values::UNSIGNED(*s $op *v as u64),
            (Values::UNSIGNED(s), Values::POINTER(v, _)) => Values::UNSIGNED(*s $op *v as u64),
            (Values::UNSIGNED(_), Values::STRUCT(_, _, _)) => panic!("type mismatch: cannot {} values of type `struct` and `number`", $op_name),
            (Values::UNSIGNED(_), Values::TYPE(_)) => panic!("type mismatch: cannot {} values of type `type` and `number`", $op_name),
            (Values::UNSIGNED(_), Values::NAME(_)) => panic!("type mismatch: cannot {} values of type `name` and `number`", $op_name),
            (Values::DECIMAL(s), Values::VOID) => Values::DECIMAL(*s),
            (Values::DECIMAL(s), Values::SIGNED(v)) => Values::DECIMAL(*s $op *v as f64),
            (Values::DECIMAL(s), Values::UNSIGNED(v)) => Values::DECIMAL(*s $op *v as f64),
            (Values::DECIMAL(s), Values::DECIMAL(v)) => Values::DECIMAL(*s $op *v),
            (Values::DECIMAL(s), Values::POINTER(v, _)) => Values::DECIMAL(*s $op *v as f64),
            (Values::DECIMAL(_), Values::STRUCT(_, _, _)) => panic!("type mismatch: cannot {} values of type `struct` and `number`", $op_name),
            (Values::DECIMAL(_), Values::TYPE(_)) => panic!("type mismatch: cannot {} values of type `type` and `number`", $op_name),
            (Values::DECIMAL(_), Values::NAME(_)) => panic!("type mismatch: cannot {} values of type `name` and `number`", $op_name),
            (Values::POINTER(p, s), Values::VOID) => Values::POINTER(*p, *s),
            (Values::POINTER(p, s), Values::SIGNED(v)) => Values::POINTER(*p $op *v as usize, *s),
            (Values::POINTER(p, s), Values::UNSIGNED(v)) => Values::POINTER(*p $op *v as usize, *s),
            (Values::POINTER(p, s), Values::DECIMAL(v)) => Values::POINTER(*p $op *v as usize, *s),
            (Values::POINTER(p, s), Values::POINTER(v, _)) => Values::POINTER(*p $op *v, *s),
            (Values::POINTER(_, _), Values::STRUCT(_, _, _)) => panic!("type mismatch: cannot {} values of type `struct` and `pointer`", $op_name),
            (Values::POINTER(_, _), Values::TYPE(_)) => panic!("type mismatch: cannot {} values of type `type` and `pointer`", $op_name),
            (Values::POINTER(_, _), Values::NAME(_)) => panic!("type mismatch: cannot {} values of type `name` and `pointer`", $op_name),
            (Values::STRUCT(_, _, _), _) => panic!("type mismatch: value of type `struct` cannot be {}", $op_plural),
            (Values::TYPE(_), _) => panic!("type mismatch: value of type `type` cannot be {}", $op_plural),
            (Values::NAME(_), _) => panic!("type mismatch: value of type `name` cannot be {}", $op_plural),
        }
    };
}

macro_rules! bitwise {
    ($self:expr, $other:expr, $op:tt, $op_name:expr, $op_plural:expr) => {
        return match($self, $other) {
            (Values::VOID, _) => Values::VOID,
            (Values::SIGNED(s), Values::VOID) => Values::SIGNED(*s),
            (Values::SIGNED(s), Values::SIGNED(v)) => Values::SIGNED(*s $op *v),
            (Values::SIGNED(s), Values::UNSIGNED(v)) => Values::SIGNED(*s $op *v as i64),
            (Values::SIGNED(s), Values::DECIMAL(v)) => Values::SIGNED(*s $op f64::to_bits(*v) as i64),
            (Values::SIGNED(s), Values::POINTER(v, _)) => Values::SIGNED(*s $op *v as i64),
            (Values::SIGNED(_), Values::STRUCT(_, _, _)) => panic!("type mismatch: cannot {} values of type `struct` and `number`", $op_name),
            (Values::SIGNED(_), Values::TYPE(_)) => panic!("type mismatch: cannot {} values of type `type` and `number`", $op_name),
            (Values::SIGNED(_), Values::NAME(_)) => panic!("type mismatch: cannot {} values of type `name` and `number`", $op_name),
            (Values::UNSIGNED(s), Values::VOID) => Values::UNSIGNED(*s),
            (Values::UNSIGNED(s), Values::SIGNED(v)) => Values::UNSIGNED(*s $op *v as u64),
            (Values::UNSIGNED(s), Values::UNSIGNED(v)) => Values::UNSIGNED(*s $op *v),
            (Values::UNSIGNED(s), Values::DECIMAL(v)) => Values::UNSIGNED(*s $op f64::to_bits(*v)),
            (Values::UNSIGNED(s), Values::POINTER(v, _)) => Values::UNSIGNED(*s $op *v as u64),
            (Values::UNSIGNED(_), Values::STRUCT(_, _, _)) => panic!("type mismatch: cannot {} values of type `struct` and `number`", $op_name),
            (Values::UNSIGNED(_), Values::TYPE(_)) => panic!("type mismatch: cannot {} values of type `type` and `number`", $op_name),
            (Values::UNSIGNED(_), Values::NAME(_)) => panic!("type mismatch: cannot {} values of type `name` and `number`", $op_name),
            (Values::DECIMAL(s), Values::VOID) => Values::DECIMAL(*s),
            (Values::DECIMAL(s), Values::SIGNED(v)) => Values::DECIMAL(f64::from_bits((f64::to_bits(*s) $op *v as u64))),
            (Values::DECIMAL(s), Values::UNSIGNED(v)) => Values::DECIMAL(f64::from_bits(f64::to_bits(*s) $op *v)),
            (Values::DECIMAL(s), Values::DECIMAL(v)) => Values::DECIMAL(f64::from_bits(f64::to_bits(*s) $op f64::to_bits(*v))),
            (Values::DECIMAL(s), Values::POINTER(v, _)) => Values::DECIMAL(f64::from_bits(f64::to_bits(*s) $op *v as u64)),
            (Values::DECIMAL(_), Values::STRUCT(_, _, _)) => panic!("type mismatch: cannot {} values of type `struct` and `number`", $op_name),
            (Values::DECIMAL(_), Values::TYPE(_)) => panic!("type mismatch: cannot {} values of type `type` and `number`", $op_name),
            (Values::DECIMAL(_), Values::NAME(_)) => panic!("type mismatch: cannot {} values of type `name` and `number`", $op_name),
            (Values::POINTER(p, s), Values::VOID) => Values::POINTER(*p, *s),
            (Values::POINTER(p, s), Values::SIGNED(v)) => Values::POINTER(*p $op *v as usize, *s),
            (Values::POINTER(p, s), Values::UNSIGNED(v)) => Values::POINTER(*p $op *v as usize, *s),
            (Values::POINTER(p, s), Values::DECIMAL(v)) => Values::POINTER(*p $op f64::to_bits(*v) as usize, *s),
            (Values::POINTER(p, s), Values::POINTER(v, _)) => Values::POINTER(*p $op *v, *s),
            (Values::POINTER(_, _), Values::STRUCT(_, _, _)) => panic!("type mismatch: cannot {} values of type `struct` and `pointer`", $op_name),
            (Values::POINTER(_, _), Values::TYPE(_)) => panic!("type mismatch: cannot {} values of type `type` and `pointer`", $op_name),
            (Values::POINTER(_, _), Values::NAME(_)) => panic!("type mismatch: cannot {} values of type `name` and `pointer`", $op_name),
            (Values::STRUCT(_, _, _), _) => panic!("type mismatch: value of type `struct` cannot be {} to", $op_plural),
            (Values::TYPE(_), _) => panic!("type mismatch: value of type `type` cannot be {} to", $op_plural),
            (Values::NAME(_), _) => panic!("type mismatch: value of type `name` cannot be {} to", $op_plural),
        }
    };
}

macro_rules! compare {
    ($self:expr, $other:expr, $op:tt) => {
        match($self, $other) {
            (Values::VOID, Values::VOID) => true,
            (Values::VOID, _) => false,
            (Values::SIGNED(_), Values::VOID) => false,
            (Values::SIGNED(s), Values::SIGNED(v)) => *s $op *v,
            (Values::SIGNED(s), Values::UNSIGNED(v)) => *s $op *v as i64,
            (Values::SIGNED(s), Values::DECIMAL(v)) => *s $op *v as i64,
            (Values::SIGNED(s), Values::POINTER(v, _)) => *s $op *v as i64,
            (Values::SIGNED(_), Values::STRUCT(_, _, _)) => false,
            (Values::SIGNED(_), Values::TYPE(_)) => false,
            (Values::SIGNED(_), Values::NAME(_)) => false,
            (Values::UNSIGNED(_), Values::VOID) => false,
            (Values::UNSIGNED(s), Values::SIGNED(v)) => *s $op *v as u64,
            (Values::UNSIGNED(s), Values::UNSIGNED(v)) => *s $op *v,
            (Values::UNSIGNED(s), Values::DECIMAL(v)) => *s $op *v as u64,
            (Values::UNSIGNED(s), Values::POINTER(v, _)) => *s $op *v as u64,
            (Values::UNSIGNED(_), Values::STRUCT(_, _, _)) => false,
            (Values::UNSIGNED(_), Values::TYPE(_)) => false,
            (Values::UNSIGNED(_), Values::NAME(_)) => false,
            (Values::DECIMAL(_), Values::VOID) => false,
            (Values::DECIMAL(s), Values::SIGNED(v)) => *s $op *v as f64,
            (Values::DECIMAL(s), Values::UNSIGNED(v)) => *s $op *v as f64,
            (Values::DECIMAL(s), Values::DECIMAL(v)) => *s $op *v,
            (Values::DECIMAL(s), Values::POINTER(v, _)) => *s $op *v as f64,
            (Values::DECIMAL(_), Values::STRUCT(_, _, _)) => false,
            (Values::DECIMAL(_), Values::TYPE(_)) => false,
            (Values::DECIMAL(_), Values::NAME(_)) => false,
            (Values::POINTER(_, _), Values::VOID) => false,
            (Values::POINTER(s, _), Values::SIGNED(v)) => *s $op *v as usize,
            (Values::POINTER(s, _), Values::UNSIGNED(v)) => *s $op *v as usize,
            (Values::POINTER(s, _), Values::DECIMAL(v)) => *s $op *v as usize,
            (Values::POINTER(s, _), Values::POINTER(v, _)) => *s $op *v,
            (Values::POINTER(_, _), Values::STRUCT(_, _, _)) => false,
            (Values::POINTER(_, _), Values::TYPE(_)) => false,
            (Values::POINTER(_, _), Values::NAME(_)) => false,
            (Values::STRUCT(_, _, _), _) => false,
            (Values::TYPE(_), _) => false,
            (Values::NAME(_), _) => false,
        }
    }
}

impl cmp::PartialEq for Values {
    fn eq(&self, other: &Self) -> bool {
        compare!(self, other, ==)
    }
}

impl cmp::PartialOrd for Values {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        let less = compare!(self, other, <);
        let greater = compare!(self, other, >);

        if less {
            return Some(Ordering::Less);
        }
        if greater {
            return Some(Ordering::Greater);
        }

        return Some(Ordering::Equal);
    }
}

// TODO: actually make sure resulting numbers can fit in the types they're supposed to be
impl Values {
    pub fn set(&mut self, other: &Values) { // basically auto-type casting
        match(self, other) {
            (Values::VOID, _) => (),
            (Values::SIGNED(s), Values::VOID) => *s = 0,
            (Values::SIGNED(s), Values::SIGNED(v)) => *s = *v,
            (Values::SIGNED(s), Values::UNSIGNED(v)) => *s = *v as i64,
            (Values::SIGNED(s), Values::DECIMAL(v)) => *s = *v as i64,
            (Values::SIGNED(s), Values::POINTER(v, _)) => *s = *v as i64,
            (Values::SIGNED(_), Values::STRUCT(_, _, _)) => panic!("type mismatch: attempted to assign value of type `struct` to variable with type `number`"),
            (Values::SIGNED(_), Values::TYPE(_)) => panic!("type mismatch: attempted to assign value of type `type` to variable with type `number`"),
            (Values::SIGNED(_), Values::NAME(_)) => panic!("type mismatch: attempted to assign value of type `name` to variable with type `number`"),
            (Values::UNSIGNED(s), Values::VOID) => *s = 0,
            (Values::UNSIGNED(s), Values::SIGNED(v)) => *s = *v as u64,
            (Values::UNSIGNED(s), Values::UNSIGNED(v)) => *s = *v,
            (Values::UNSIGNED(s), Values::DECIMAL(v)) => *s = *v as u64,
            (Values::UNSIGNED(s), Values::POINTER(v, _)) => *s = *v as u64,
            (Values::UNSIGNED(_), Values::STRUCT(_, _, _)) => panic!("type mismatch: attempted to assign value of type `struct` to variable with type `number`"),
            (Values::UNSIGNED(_), Values::TYPE(_)) => panic!("type mismatch: attempted to assign value of type `type` to variable with type `number`"),
            (Values::UNSIGNED(_), Values::NAME(_)) => panic!("type mismatch: attempted to assign value of type `name` to variable with type `number`"),
            (Values::DECIMAL(s), Values::VOID) => *s = 0.0,
            (Values::DECIMAL(s), Values::SIGNED(v)) => *s = *v as f64,
            (Values::DECIMAL(s), Values::UNSIGNED(v)) => *s = *v as f64,
            (Values::DECIMAL(s), Values::DECIMAL(v)) => *s = *v,
            (Values::DECIMAL(s), Values::POINTER(v, _)) => *s = *v as f64,
            (Values::DECIMAL(_), Values::STRUCT(_, _, _)) => panic!("type mismatch: attempted to assign value of type `struct` to variable with type `number`"),
            (Values::DECIMAL(_), Values::TYPE(_)) => panic!("type mismatch: attempted to assign value of type `type` to variable with type `number`"),
            (Values::DECIMAL(_), Values::NAME(_)) => panic!("type mismatch: attempted to assign value of type `name` to variable with type `number`"),
            (Values::POINTER(p, _), Values::VOID) => *p = 0,
            (Values::POINTER(p, _), Values::SIGNED(v)) => *p = *v as usize,
            (Values::POINTER(p, _), Values::UNSIGNED(v)) => *p = *v as usize,
            (Values::POINTER(p, _), Values::DECIMAL(v)) => *p = *v as usize,
            (Values::POINTER(p, s), Values::POINTER(v, o)) => { *p = *v; *s = *o; }
            (Values::POINTER(p, _), Values::STRUCT(_, _, v)) => { *p = *v; },
            (Values::POINTER(_, _), Values::TYPE(_)) => panic!("type mismatch: attempted to assign value of type `type` to variable with type `pointer`"),
            (Values::POINTER(_, _), Values::NAME(_)) => panic!("type mismatch: attempted to assign value of type `name` to variable with type `pointer`"),
            (Values::STRUCT(_, _, _), Values::VOID) => todo!(),
            (Values::STRUCT(_, _, _), Values::SIGNED(_)) => panic!("type mismatch: attempted to assign value of type `number` to variable with type `struct`"),
            (Values::STRUCT(_, _, _), Values::UNSIGNED(_)) => panic!("type mismatch: attempted to assign value of type `number` to variable with type `struct`"),
            (Values::STRUCT(_, _, _), Values::DECIMAL(_)) => panic!("type mismatch: attempted to assign value of type `number` to variable with type `struct`"),
            (Values::STRUCT(_, _, _), Values::POINTER(_, _)) => panic!("type mismatch: attempted to assign value of type `pointer` to variable with type `struct`"),
            (Values::STRUCT(m, n, p), Values::STRUCT(om, on, op)) => { *m = om.clone(); *n = on.clone(); *p = *op; },
            (Values::STRUCT(_, _, _), Values::TYPE(_)) => panic!("type mismatch: attempted to assign value of type `type` to variable with type `struct`"),
            (Values::STRUCT(_, _, _), Values::NAME(_)) => panic!("type mismatch: attempted to assign value of type `name` to variable with type `struct`"),
            (Values::TYPE(_), Values::VOID) => todo!(),
            (Values::TYPE(_), Values::SIGNED(_)) => panic!("type mismatch: attempted to assign value of type `number` to variable with type `type`"),
            (Values::TYPE(_), Values::UNSIGNED(_)) => panic!("type mismatch: attempted to assign value of type `number` to variable with type `type`"),
            (Values::TYPE(_), Values::DECIMAL(_)) => panic!("type mismatch: attempted to assign value of type `number` to variable with type `type`"),
            (Values::TYPE(_), Values::POINTER(_, _)) => panic!("type mismatch: attempted to assign value of type `pointer` to variable with type `type`"),
            (Values::TYPE(_), Values::STRUCT(_, _, _)) => panic!("type mismatch: attempted to assign value of type `struct` to variable with type `type`"),
            (Values::TYPE(s), Values::TYPE(v)) => *s = v.clone(),
            (Values::TYPE(_), Values::NAME(_)) => panic!("type mismatch: attempted to assign value of type `name` to variable with type `type`"),
            (Values::NAME(s), Values::VOID) => *s = String::new(),
            (Values::NAME(_), Values::SIGNED(_)) => panic!("type mismatch: attempted to assign value of type `number` to variable with type `name`"),
            (Values::NAME(_), Values::UNSIGNED(_)) => panic!("type mismatch: attempted to assign value of type `number` to variable with type `name`"),
            (Values::NAME(_), Values::DECIMAL(_)) => panic!("type mismatch: attempted to assign value of type `number` to variable with type `name`"),
            (Values::NAME(_), Values::POINTER(_, _)) => panic!("type mismatch: attempted to assign value of type `pointer` to variable with type `name`"),
            (Values::NAME(_), Values::STRUCT(_, _, _)) => panic!("type mismatch: attempted to assign value of type `struct` to variable with type `name`"),
            (Values::NAME(_), Values::TYPE(_)) => panic!("type mismatch: attempted to assign value of type `type` to variable with type `name`"),
            (Values::NAME(s), Values::NAME(v)) => *s = v.clone(),
        }    
    }

    // math operations
    pub fn add(&self, other: &Values) -> Values {
        math!(self, other, +, "add", "added");
    }

    pub fn sub(&self, other: &Values) -> Values {
        math!(self, other, -, "subtract", "subtracted");
    }
    
    pub fn mul(&self, other: &Values) -> Values {
        math!(self, other, *, "subtract", "subtracted");
    }
    
    pub fn div(&self, other: &Values) -> Values {
        math!(self, other, /, "divide", "divided");
    }

    pub fn modulo(&self, other: &Values) -> Values {
        math!(self, other, %, "modulo", "modulo");
    }

    // bitwise operations
    pub fn and(&self, other: &Values) -> Values {
        bitwise!(self, other, &, "AND", "ANDed");
    }
    
    pub fn or(&self, other: &Values) -> Values {
        bitwise!(self, other, |, "OR", "ORed");
    }
    
    pub fn xor(&self, other: &Values) -> Values {
        bitwise!(self, other, ^, "XOR", "XORed");
    }
    
    pub fn not(&self) -> Values {
        match self {
            Values::VOID => Values::VOID,
            Values::SIGNED(v) => Values::SIGNED(!v),
            Values::UNSIGNED(v) => Values::UNSIGNED(!v),
            Values::DECIMAL(v) => Values::DECIMAL(f64::from_bits(!f64::to_bits(*v))),
            Values::POINTER(v, s) => Values::POINTER(!v, *s),
            Values::STRUCT(_, _, _) => panic!("type mismatch: cannot NOT a value with type `struct`"),
            Values::TYPE(_) => panic!("type mismatch: cannot NOT a value with type `type`"),
            Values::NAME(_) => panic!("type mismatch: cannot NOT a value with type `name`"),
        }
    }

    pub fn lsh(&self, other: &Values) -> Values {
        bitwise!(self, other, <<, "left shift", "left shifted");
    }
    
    pub fn rsh(&self, other: &Values) -> Values {
        bitwise!(self, other, >>, "right shift", "right shifted");
    }
}
