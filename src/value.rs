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
    STRUCT, // TODO
    TYPE(Type),
    NAME(String),
}

#[derive(Debug, Clone)]
pub struct Value {
    pub typ: Type,
    pub val: Values,
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
            (Values::SIGNED(_), Values::STRUCT) => panic!("cannot {} struct and a number", $op_name),
            (Values::SIGNED(_), Values::TYPE(_)) => panic!("cannot {} type and a number", $op_name),
            (Values::SIGNED(_), Values::NAME(_)) => panic!("cannot {} name and a number", $op_name),
            (Values::UNSIGNED(s), Values::VOID) => Values::UNSIGNED(*s),
            (Values::UNSIGNED(s), Values::SIGNED(v)) => Values::UNSIGNED(*s $op *v as u64),
            (Values::UNSIGNED(s), Values::UNSIGNED(v)) => Values::UNSIGNED(*s $op *v),
            (Values::UNSIGNED(s), Values::DECIMAL(v)) => Values::UNSIGNED(*s $op *v as u64),
            (Values::UNSIGNED(s), Values::POINTER(v, _)) => Values::UNSIGNED(*s $op *v as u64),
            (Values::UNSIGNED(_), Values::STRUCT) => panic!("cannot {} struct and a number", $op_name),
            (Values::UNSIGNED(_), Values::TYPE(_)) => panic!("cannot {} type and a number", $op_name),
            (Values::UNSIGNED(_), Values::NAME(_)) => panic!("cannot {} name and a number", $op_name),
            (Values::DECIMAL(s), Values::VOID) => Values::DECIMAL(*s),
            (Values::DECIMAL(s), Values::SIGNED(v)) => Values::DECIMAL(*s $op *v as f64),
            (Values::DECIMAL(s), Values::UNSIGNED(v)) => Values::DECIMAL(*s $op *v as f64),
            (Values::DECIMAL(s), Values::DECIMAL(v)) => Values::DECIMAL(*s $op *v),
            (Values::DECIMAL(s), Values::POINTER(v, _)) => Values::DECIMAL(*s $op *v as f64),
            (Values::DECIMAL(_), Values::STRUCT) => panic!("cannot {} struct and a number", $op_name),
            (Values::DECIMAL(_), Values::TYPE(_)) => panic!("cannot {} type and a number", $op_name),
            (Values::DECIMAL(_), Values::NAME(_)) => panic!("cannot {} name and a number", $op_name),
            (Values::POINTER(p, s), Values::VOID) => Values::POINTER(*p, *s),
            (Values::POINTER(p, s), Values::SIGNED(v)) => Values::POINTER(*p $op *v as usize, *s),
            (Values::POINTER(p, s), Values::UNSIGNED(v)) => Values::POINTER(*p $op *v as usize, *s),
            (Values::POINTER(p, s), Values::DECIMAL(v)) => Values::POINTER(*p $op *v as usize, *s),
            (Values::POINTER(p, s), Values::POINTER(v, _)) => Values::POINTER(*p $op *v, *s),
            (Values::POINTER(_, _), Values::STRUCT) => panic!("cannot {} struct and a pointer", $op_name),
            (Values::POINTER(_, _), Values::TYPE(_)) => panic!("cannot {} type and a pointer", $op_name),
            (Values::POINTER(_, _), Values::NAME(_)) => panic!("cannot {} name and a pointer", $op_name),
            (Values::STRUCT, _) => panic!("struct cannot be {} to", $op_plural),
            (Values::TYPE(_), _) => panic!("type cannot be {} to", $op_plural),
            (Values::NAME(_), _) => panic!("name cannot be {} to", $op_plural),
        }
    };
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
            (Values::SIGNED(_), Values::STRUCT) => panic!("cannot set a number value as a struct"),
            (Values::SIGNED(_), Values::TYPE(_)) => panic!("cannot set a number value as a type"),
            (Values::SIGNED(_), Values::NAME(_)) => panic!("cannot set a number value as a name"),
            (Values::UNSIGNED(s), Values::VOID) => *s = 0,
            (Values::UNSIGNED(s), Values::SIGNED(v)) => *s = *v as u64,
            (Values::UNSIGNED(s), Values::UNSIGNED(v)) => *s = *v,
            (Values::UNSIGNED(s), Values::DECIMAL(v)) => *s = *v as u64,
            (Values::UNSIGNED(s), Values::POINTER(v, _)) => *s = *v as u64,
            (Values::UNSIGNED(_), Values::STRUCT) => panic!("cannot set a number value as a struct"),
            (Values::UNSIGNED(_), Values::TYPE(_)) => panic!("cannot set a number value as a type"),
            (Values::UNSIGNED(_), Values::NAME(_)) => panic!("cannot set a number value as a name"),
            (Values::DECIMAL(s), Values::VOID) => *s = 0.0,
            (Values::DECIMAL(s), Values::SIGNED(v)) => *s = *v as f64,
            (Values::DECIMAL(s), Values::UNSIGNED(v)) => *s = *v as f64,
            (Values::DECIMAL(s), Values::DECIMAL(v)) => *s = *v,
            (Values::DECIMAL(s), Values::POINTER(v, _)) => *s = *v as f64,
            (Values::DECIMAL(_), Values::STRUCT) => panic!("cannot set a number value as a struct"),
            (Values::DECIMAL(_), Values::TYPE(_)) => panic!("cannot set a number value as a type"),
            (Values::DECIMAL(_), Values::NAME(_)) => panic!("cannot set a number value as a name"),
            (Values::POINTER(p, _), Values::VOID) => *p = 0,
            (Values::POINTER(p, _), Values::SIGNED(v)) => *p = *v as usize,
            (Values::POINTER(p, _), Values::UNSIGNED(v)) => *p = *v as usize,
            (Values::POINTER(p, _), Values::DECIMAL(v)) => *p = *v as usize,
            (Values::POINTER(p, s), Values::POINTER(v, o)) => { *p = *v; *s = *o; }
            (Values::POINTER(_, _), Values::STRUCT) => panic!("cannot set a pointer value as a struct"),
            (Values::POINTER(_, _), Values::TYPE(_)) => panic!("cannot set a pointer value as a type"),
            (Values::POINTER(_, _), Values::NAME(_)) => panic!("cannot set a pointer value as a name"),
            (Values::STRUCT, Values::VOID) => todo!(),
            (Values::STRUCT, Values::SIGNED(_)) => todo!(),
            (Values::STRUCT, Values::UNSIGNED(_)) => todo!(),
            (Values::STRUCT, Values::DECIMAL(_)) => todo!(),
            (Values::STRUCT, Values::POINTER(_, _)) => todo!(),
            (Values::STRUCT, Values::STRUCT) => todo!(),
            (Values::STRUCT, Values::TYPE(_)) => todo!(),
            (Values::STRUCT, Values::NAME(_)) => todo!(),
            (Values::TYPE(_), Values::VOID) => todo!(),
            (Values::TYPE(_), Values::SIGNED(_)) => todo!(),
            (Values::TYPE(_), Values::UNSIGNED(_)) => todo!(),
            (Values::TYPE(_), Values::DECIMAL(_)) => todo!(),
            (Values::TYPE(_), Values::POINTER(_, _)) => todo!(),
            (Values::TYPE(_), Values::STRUCT) => todo!(),
            (Values::TYPE(s), Values::TYPE(v)) => *s = v.clone(),
            (Values::TYPE(_), Values::NAME(_)) => todo!(),
            (Values::NAME(s), Values::VOID) => *s = String::new(),
            (Values::NAME(_), Values::SIGNED(_)) => panic!("cannot set a name value as a number"),
            (Values::NAME(_), Values::UNSIGNED(_)) => panic!("cannot set a name value as a number"),
            (Values::NAME(_), Values::DECIMAL(_)) => panic!("cannot set a name value as a number"),
            (Values::NAME(_), Values::POINTER(_, _)) => panic!("cannot set a name value as a pointer"),
            (Values::NAME(_), Values::STRUCT) => panic!("cannot set a name value as a struct"),
            (Values::NAME(_), Values::TYPE(_)) => panic!("cannot set a name value as a type"),
            (Values::NAME(s), Values::NAME(v)) => *s = v.clone(),
        }    
    }

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
}